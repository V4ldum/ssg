use crate::pipeline::{Page, TransformerPriority};
use crate::traits::Transformer;
use anyhow::{bail, Result};

pub struct MarkdownTransformer {}

impl Transformer for MarkdownTransformer {
    fn should_transform(&self, page: &Page) -> bool {
        page.url
            .extension()
            .map(|extension| extension.to_string_lossy() == "md")
            .unwrap_or(false)
    }

    fn transformer_priority(&self) -> TransformerPriority {
        TransformerPriority::Highest
    }

    fn transform(&self, page: &mut Page) -> Result<()> {
        let Ok(html) = markdown::to_html_with_options(&page.content, &markdown::Options::gfm())
        else {
            bail!("Failed to convert {} to HTML, skipping", page.url.display())
        };

        page.content = html;
        page.url.set_extension("html");

        Ok(())
    }
}

impl MarkdownTransformer {
    pub fn new() -> Self {
        MarkdownTransformer {}
    }
}
