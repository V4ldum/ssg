use crate::pipeline::Page;
use crate::traits::Loader;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use std::collections::HashMap;
use std::path::Path;

pub struct MarkdownLoader {}

impl Loader for MarkdownLoader {
    fn should_load(&self, path: &Path) -> bool {
        path.extension()
            .map(|extension| extension.to_string_lossy() == "md")
            .unwrap_or(false)
    }

    fn load(&self, path: &Path, content: &str) -> Page {
        let matter = Matter::<YAML>::new();
        let parsed = matter.parse(content);

        let data: HashMap<String, String> = parsed
            .data
            .and_then(|data| data.deserialize().ok())
            .unwrap_or_default();

        Page {
            url: path.into(),
            data,
            content: parsed.content,
        }
    }
}

impl MarkdownLoader {
    pub fn new() -> Self {
        MarkdownLoader {}
    }
}
