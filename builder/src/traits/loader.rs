use crate::pipeline::Page;
use std::path::Path;

pub trait Loader {
    fn should_load(&self, path: &Path) -> bool;

    fn load(&self, path: &Path, content: &str) -> Page;
}
