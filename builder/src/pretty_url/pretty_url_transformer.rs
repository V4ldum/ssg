use crate::pipeline::{Page, TransformerPriority};
use crate::traits::Transformer;
use anyhow::{bail, Result};

pub struct PrettyUrlTransformer {}

impl Transformer for PrettyUrlTransformer {
    fn should_transform(&self, page: &Page) -> bool {
        let extension = page
            .url
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default();
        let filename = page
            .url
            .file_name()
            .and_then(|filename| filename.to_str())
            .unwrap_or_default();

        extension == "html" && filename != "index.html"
    }

    fn transformer_priority(&self) -> TransformerPriority {
        TransformerPriority::Lowest
    }

    fn transform(&self, page: &mut Page) -> Result<()> {
        let Some((parent, file_stem)) = page.url.parent().and_then(|parent| {
            page.url.file_stem().map(|file_stem| (parent, file_stem)) //
        }) else {
            bail!("Failed to prettify {}, skipping", page.url.display());
        };

        let new_path = parent.join(file_stem).join("index.html");
        page.url = new_path;

        Ok(())
    }
}

impl PrettyUrlTransformer {
    pub fn new() -> Self {
        PrettyUrlTransformer {}
    }
}
