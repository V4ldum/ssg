use crate::traits::Picker;
use std::path::Path;

pub enum FileFilter<'fr> {
    Name(&'fr str),
    Extension(&'fr str),
    Directory(&'fr str),
}

pub struct FilePicker<'fp> {
    filter: FileFilter<'fp>,
}

impl Picker for FilePicker<'_> {
    fn should_pick(&self, file: &Path) -> bool {
        match self.filter {
            FileFilter::Name(name) => file
                .file_name()
                .map(|filename| filename.to_string_lossy() == name)
                .unwrap_or(false),
            FileFilter::Extension(ext) => file
                .extension()
                .map(|extension| extension.to_string_lossy() == ext)
                .unwrap_or(false),
            FileFilter::Directory(dir) => {
                // Sanitize dir: remove leading and trailing slash and backslash, and replace potential backslashes with
                // slashes to simplify comparison
                let dir = dir
                    // Remove leading slash or backslash
                    .strip_prefix("/")
                    .and_then(|dir| dir.strip_prefix("\\"))
                    .unwrap_or(dir)
                    // Remove trailing slash or backslash
                    .strip_suffix("/")
                    .and_then(|dir| dir.strip_suffix("\\"))
                    .unwrap_or(dir)
                    // Convert backlashes to slashes
                    .replace("\\", "/");

                file.parent()
                    .map(|parent| parent.to_string_lossy().replace("\\", "/").contains(&dir))
                    .unwrap_or(false)
            }
        }
    }
}

impl<'fp> FilePicker<'fp> {
    pub fn new(filter: FileFilter<'fp>) -> Self {
        FilePicker { filter }
    }
}
