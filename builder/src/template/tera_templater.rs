use crate::pipeline::{Page, PipelineContext};
use crate::traits::Templater;
use anyhow::{Context, Result};
use std::path::Path;
use tera::Tera;

#[derive(Default)]
pub struct TeraTemplater {
    global_tera: Tera,
}

impl Templater for TeraTemplater {
    fn name(&self) -> &str {
        "TeraTemplater"
    }
    fn should_template(&self, page: &Page) -> bool {
        page.url
            .extension()
            .map(|extension| extension.to_string_lossy() == "tera")
            .unwrap_or(false)
            || page.data.contains_key("template")
    }

    fn template(
        &self,
        current_page_index: usize,
        pipeline_context: &mut PipelineContext,
    ) -> Result<()> {
        let current_page = &pipeline_context.pages[current_page_index];

        // Call the correct templater implementation
        let rendered = match &current_page.data.get("template") {
            // Page specifies its template we run that one
            Some(template) => self.template(current_page, template, pipeline_context)?,
            // Page doesn't specify its template so we run it in a generic context
            None => self.template_generic(current_page, pipeline_context)?,
        };

        let current_page = &mut pipeline_context.pages[current_page_index];
        // Remove empty lines artifacts from the templater
        let string = rendered
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<&str>>()
            .join("\n");

        current_page.content = string;
        current_page.url.set_extension("html");

        Ok(())
    }
}

impl TeraTemplater {
    pub fn new() -> Self {
        // todo custom parameter
        let templates_dir = Path::new("includes/templates");
        let result = Tera::new(&format!("{}/*", templates_dir.to_string_lossy()));

        match result {
            Err(err) => {
                panic!("Error when building the templater : {:?}", err.kind)
            }
            Ok(tera) => TeraTemplater { global_tera: tera },
        }
    }

    fn template(
        &self,
        current_page: &Page,
        template_name: &str,
        pipeline_context: &PipelineContext,
    ) -> Result<String> {
        let title = current_page.data.get("title").context(format!(
            "Attribute \"title\" was not found for {}, skipping",
            current_page.url.display()
        ))?;
        let mut context = tera::Context::new();

        context.insert("components", &pipeline_context.components);
        context.insert("title", title);
        context.insert("content", &current_page.content);

        let rendered = self
            .global_tera
            .render(template_name, &context)
            .context(format!(
                "Template \"{}\" was not found for {}, skipping",
                template_name,
                current_page.url.display()
            ))?;

        Ok(rendered)
    }

    fn template_generic(
        &self,
        current_page: &Page,
        pipeline_context: &PipelineContext,
    ) -> Result<String> {
        let mut tera = Tera::default();
        let mut context = tera::Context::new();

        let generic_template_name = "generic";
        tera.add_raw_template(generic_template_name, &current_page.content)
            .context(format!(
                "Invalid template for {}, skipping",
                current_page.url.display()
            ))?;

        // Hacky way to remove index.html from the templater to have better URLs
        // Not ideal, but I did not find a better way to do it
        let clone: Vec<_> = pipeline_context
            .pages
            .iter()
            .cloned()
            .map(|mut page| {
                let filename = page
                    .url
                    .file_name()
                    .and_then(|filename| filename.to_str())
                    .unwrap_or_default();

                if filename == "index.html" {
                    if let Some(parent) = page.url.parent() {
                        page.url = parent.into();
                    }
                }

                page
            })
            .collect();

        context.insert("pages", &clone);
        context.insert("components", &pipeline_context.components);
        // This only returns an error if the template was not found. This should never happen.
        let rendered = tera
            .render(generic_template_name, &context)
            .expect("Invalid template or variable not found"); // todo skip templating file instead ?

        Ok(rendered)
    }
}
