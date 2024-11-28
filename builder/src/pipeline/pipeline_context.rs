use crate::pipeline::Asset;
use crate::pipeline::Page;
use std::collections::HashMap;

#[derive(Default)]
pub struct PipelineContext {
    pub(crate) components: HashMap<String, String>,
    pub(crate) pages: Vec<Page>,
    pub(crate) assets: Vec<Asset>,
}
