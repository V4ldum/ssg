use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Debug, Default, Clone, PartialEq)]
pub struct Page {
    pub url: PathBuf,
    pub data: HashMap<String, String>,
    pub content: String,
}
