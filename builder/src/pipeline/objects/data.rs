use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Debug, Default)]
pub(crate) struct Data {
    pub path: PathBuf,
    pub data: HashMap<String, String>,
}
