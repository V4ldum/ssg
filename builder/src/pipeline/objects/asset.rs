use std::path::PathBuf;

pub(crate) struct Asset {
    pub path: PathBuf,
    pub content: AssetContent,
}

#[derive(Debug)]
pub(crate) enum AssetContent {
    String(String),
    Binary(Vec<u8>),
}

impl From<AssetContent> for Vec<u8> {
    fn from(value: AssetContent) -> Self {
        match value {
            AssetContent::String(content) => content.into_bytes(),
            AssetContent::Binary(content) => content,
        }
    }
}
