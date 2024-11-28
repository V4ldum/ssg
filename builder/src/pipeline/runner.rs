use crate::pipeline::{Asset, AssetContent, Data, Page, PipelineContext, TransformerPriority};
use crate::traits::{Finisher, Loader, Picker, Templater, Transformer};
use crate::{info, progress, warn};
use anyhow::Result;
use gray_matter::engine::TOML;
use gray_matter::Matter;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, mem};
use walkdir::WalkDir;

#[derive(Default)]
pub struct SSGPipelineRunner {
    pub(crate) pickers: Vec<Box<dyn Picker>>,
    pub(crate) loaders: Vec<Box<dyn Loader>>,
    pub(crate) transformers: Vec<Box<dyn Transformer>>,
    pub(crate) finishers: Vec<Box<dyn Finisher>>,
    pub(crate) templater: Option<Box<dyn Templater>>,
    pub(crate) data: Vec<Data>,
    pub context: PipelineContext,
}

impl SSGPipelineRunner {
    fn clean_build_folder(&self, build_dir: &Path) {
        info!("Cleaning build folder");

        let _ignore = fs::remove_dir_all(build_dir);

        info!();
    }

    fn index_components(&mut self, components_dir: &Path) {
        info!("Indexing components");

        self.context.components = WalkDir::new(components_dir)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path().to_path_buf();

                if path.is_file() && path.extension().unwrap().to_string_lossy() == "html" {
                    progress!("Indexing: {}", path.display());

                    Some(path)
                } else {
                    None
                }
            })
            .filter_map(|path| {
                let name = path.file_stem().and_then(|stem| stem.to_str());
                let content = fs::read_to_string(&path);

                match (name, content) {
                    (Some(name), Ok(content)) => Some((name.into(), content)),
                    _ => {
                        warn!("Failed to read {}, skipping", path.display());
                        None
                    }
                }
            })
            .collect();

        info!();
    }

    fn index_data(&mut self, source_dir: &Path) {
        info!("Indexing data files");

        self.data = WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path().to_path_buf();

                if path.is_file() {
                    let filename = path.file_name().and_then(|filename| filename.to_str())?;

                    if filename == "_data.toml" {
                        progress!("Indexing: {}", path.display());

                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .filter_map(|path| {
                // Read data
                let Ok(content) = fs::read_to_string(&path) else {
                    warn!("Failed to read {}, skipping", path.display());
                    return None;
                };

                let matter = Matter::<TOML>::new();
                let parsed = matter.parse(&format!("---\n{content}\n---"));

                // Stripping source directory from the path
                let Ok(path) = path.strip_prefix(source_dir).map(|path| path.to_path_buf()) else {
                    // if you can't strip source_dir, it means the file is not inside the source directory,
                    // we don't want this file so we skip it.
                    return None;
                };

                let data: HashMap<String, String> = parsed
                    .data
                    .and_then(|data| data.deserialize().ok())
                    .unwrap_or_default();

                Some(Data { path, data })
            })
            .collect();

        info!();
    }

    fn pick_files(&self, source_dir: &Path) -> Vec<PathBuf> {
        info!("Picking all files");

        let mut source_files: Vec<PathBuf> = WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path().to_path_buf();

                if path.is_file() {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        if !self.pickers.is_empty() {
            // Remove files that are not to be picked
            source_files.retain(|file| {
                self.pickers.iter().any(|picker| {
                    let should_pick = picker.should_pick(file);

                    if should_pick {
                        progress!("Picking: {}", file.display());
                    }

                    should_pick
                })
            });
        }

        println!();
        source_files
    }

    fn clean_file_path(&self, path: &Path, source_dir: &Path) -> PathBuf {
        // Stripping source directory from the path
        let mut path = path
            .strip_prefix(source_dir)
            .map(|path| path.to_path_buf())
            .expect("All files should be inside \"source_dir\"");

        // Replacing whitespace with dashes in filename and lowercase it
        let file_name = path
            .file_name()
            .expect("No invalid paths should get here")
            .to_string_lossy()
            .to_ascii_lowercase()
            .replace(" ", "-");

        path.set_file_name(file_name);
        path
    }

    fn loading_files(&mut self, source_files: &Vec<PathBuf>, source_dir: &Path) {
        info!("Loading file content");

        for path in source_files {
            // Find the first loader that should load this file
            let loader = self.loaders.iter().find(|&loader| loader.should_load(path));

            match loader {
                // A loader was found, this is a page
                Some(loader) => {
                    progress!("Loading: {}", path.display());

                    // Read file content
                    // If this is a page we assume it's a string
                    if let Ok(content) = fs::read_to_string(path) {
                        let mut page = loader.load(path, &content);
                        page.url = self.clean_file_path(path, source_dir);

                        // Adding data file's data to the file, if found
                        if let Some(file_parent_dir) = page.url.parent() {
                            if let Some(data_file) = self
                                .data
                                .iter()
                                .find(|&data| data.path.parent() == Some(file_parent_dir))
                            {
                                for (key, value) in &data_file.data {
                                    // Only add the value if it was not already set in the file's front-matter
                                    // The file's front-matter takes precedence over folder data file
                                    page.data.entry(key.clone()).or_insert(value.clone());
                                }
                            }
                        }

                        // Updating path if base_path is set
                        if let Some(base_path) = page.data.get("base_path") {
                            if !base_path.is_empty() {
                                if let Some(file_parent) = page.url.parent() {
                                    // Remove the leading / if there is one because it would point to the root of the FS
                                    let base_path =
                                        Path::new(base_path.strip_prefix("/").unwrap_or(base_path));

                                    // Strip the parent directories from the file
                                    let new_path =
                                        page.url.strip_prefix(file_parent).unwrap_or(&page.url);

                                    // Join the new directories set in base_path
                                    let new_path = base_path.join(new_path);

                                    page.url = new_path;
                                }
                            }
                        }

                        self.context.pages.push(page);
                    } else {
                        warn!("Failed to load {}, skipping", path.display());
                    };
                }
                // No loader found, loading this as an asset
                None => {
                    progress!(
                        "No loader found for {}, it will be considered an asset",
                        path.display()
                    );

                    // Read file content
                    // We need to test both string and binary
                    if let Ok(content) = fs::read_to_string(path)
                        .map(AssetContent::String)
                        .or_else(|_| fs::read(path).map(AssetContent::Binary))
                    {
                        self.context.assets.push(Asset {
                            path: self.clean_file_path(path, source_dir),
                            content,
                        })
                    } else {
                        warn!("Failed to load {}, skipping", path.display());
                    };
                }
            }
        }

        info!();
    }

    fn transform_file_with_priority(
        &self,
        page: &mut Page,
        priority: TransformerPriority,
    ) -> Result<()> {
        let transformers = self
            .transformers
            .iter()
            .filter(|&transformer| transformer.transformer_priority() == priority);

        for transformer in transformers {
            if transformer.should_transform(page) {
                transformer.transform(page)?;
            }
        }

        Ok(())
    }
    fn transform_files(&mut self) {
        info!("Transforming files");

        let mut pages = mem::take(&mut self.context.pages);

        let mut i = 0;
        while i < pages.len() {
            let page = &mut pages[i];
            progress!("Transforming {}", page.url.display());

            let res = self
                .transform_file_with_priority(page, TransformerPriority::Highest)
                .and(self.transform_file_with_priority(page, TransformerPriority::High))
                .and(self.transform_file_with_priority(page, TransformerPriority::Normal))
                .and(self.transform_file_with_priority(page, TransformerPriority::Low))
                .and(self.transform_file_with_priority(page, TransformerPriority::Lowest));

            if let Err(err) = res {
                warn!("{err}");
                pages.remove(i);
                // We don't increment when we have an err to avoid jumping over items after having removed one
            } else {
                i += 1;
            }
        }

        self.context.pages = mem::take(&mut pages);

        info!();
    }

    fn template_files(&mut self) {
        info!("Running templater");

        if let Some(templater) = &self.templater {
            let mut i = 0;
            while i < self.context.pages.len() {
                if templater.should_template(&self.context.pages[i]) {
                    let page = &self.context.pages[i];
                    progress!(
                        "Templating {} with {}",
                        page.url.display(),
                        templater.name()
                    );

                    let res = templater.template(i, &mut self.context);
                    if let Err(err) = res {
                        warn!("{err}");
                        self.context.pages.remove(i);
                        continue; // skip incrementing "i" to avoid jumping over values in the array
                    }
                }
                i += 1;
            }
        } else {
            progress!("No templater found, skipping")
        }

        info!();
    }

    fn run_finishers(&self) {
        info!("Running finishers");

        for finisher in &self.finishers {
            finisher.finish();
        }

        info!();
    }

    fn create_intermediary_folders(&self, path: &Path, destination_path: &Path) {
        // Create all intermediary folders if necessary
        if let Some(parent_dir) = destination_path.parent() {
            fs::create_dir_all(parent_dir).unwrap_or_else(|_| {
                panic!(
                    "Unable to write {}. Check the permissions of your build directory.",
                    path.display()
                )
            });
        }
    }

    fn write_to_disk(&mut self, build_dir: &Path) {
        info!("Writing assets");

        let assets = mem::take(&mut self.context.assets);
        for asset in assets {
            progress!("Writing {}", asset.path.display());

            let destination_path = build_dir.join(&asset.path);
            self.create_intermediary_folders(&asset.path, &destination_path);

            let content: Vec<u8> = asset.content.into();
            fs::write(&destination_path, content)
                .unwrap_or_else(|_| warn!("Failed to write {:?} to disk", asset.path));
        }

        info!();
        info!("Writing pages");

        for page in &self.context.pages {
            progress!("Writing {}", page.url.display());

            let destination_path = build_dir.join(&page.url);
            self.create_intermediary_folders(&page.url, &destination_path);

            fs::write(&destination_path, &page.content)
                .unwrap_or_else(|_| warn!("Failed to write {:?} to disk", page.url));
        }

        info!();
    }

    pub fn run(mut self) {
        // todo custom parameters
        let source_dir = Path::new("source");
        let build_dir = Path::new("build");
        let includes_dir = Path::new("includes"); // todo care for template dir when this becomes user-controlled

        self.clean_build_folder(build_dir);
        self.index_components(&includes_dir.join("components"));
        self.index_data(source_dir);
        let source_files = self.pick_files(source_dir);
        self.loading_files(&source_files, source_dir);
        self.transform_files();
        self.template_files();
        self.run_finishers();
        self.write_to_disk(build_dir);
    }
}
