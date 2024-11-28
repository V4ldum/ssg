use std::path::Path;

pub trait Picker {
    fn should_pick(&self, file: &Path) -> bool;
}
