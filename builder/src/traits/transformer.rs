use crate::pipeline::{Page, TransformerPriority};
use anyhow::Result;

pub trait Transformer {
    fn should_transform(&self, page: &Page) -> bool;

    fn transformer_priority(&self) -> TransformerPriority {
        TransformerPriority::Normal
    }

    fn transform(&self, page: &mut Page) -> Result<()>;
}
