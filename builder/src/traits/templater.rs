use crate::pipeline::{Page, PipelineContext};
use anyhow::Result;

pub trait Templater {
    fn name(&self) -> &str;

    fn should_template(&self, page: &Page) -> bool;

    fn template(
        &self,
        current_page_index: usize,
        pipeline_context: &mut PipelineContext,
    ) -> Result<()>;
}
