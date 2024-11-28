use crate::pipeline::Page;
use crate::traits::Loader;
use std::path::Path;

pub struct TeraLoader {}

impl Loader for TeraLoader {
    fn should_load(&self, path: &Path) -> bool {
        path.extension()
            .map(|extension| extension.to_string_lossy() == "tera")
            .unwrap_or(false)
    }

    fn load(&self, path: &Path, content: &str) -> Page {
        // todo HTML FrontMatter ?

        Page {
            url: path.into(),
            content: content.into(),
            ..Default::default()
        }
    }
}

impl TeraLoader {
    pub fn new() -> Self {
        TeraLoader {}
    }
}
