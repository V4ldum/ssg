use crate::pretty_url::PrettyUrlTransformer;
use crate::traits::{Plugin, Transformer};

#[derive(Default)]
pub struct PrettyUrlPlugin {}

impl Plugin for PrettyUrlPlugin {
    fn transformers(&self) -> Vec<Box<dyn Transformer>> {
        vec![Box::new(PrettyUrlTransformer::new())]
    }
}

impl PrettyUrlPlugin {
    pub fn new() -> Self {
        PrettyUrlPlugin {}
    }
}
