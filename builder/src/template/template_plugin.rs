use crate::pickers::file_picker::FileFilter::Extension;
use crate::pickers::FilePicker;
use crate::template::TeraLoader;
use crate::template::TeraTemplater;
use crate::traits::Plugin;
use crate::traits::Loader;
use crate::traits::{Picker, Templater};

#[derive(Default)]
pub struct TemplatePlugin {}

impl Plugin for TemplatePlugin {
    fn pickers(&self) -> Vec<Box<dyn Picker>> {
        vec![Box::new(FilePicker::new(Extension("tera")))]
    }

    fn loaders(&self) -> Vec<Box<dyn Loader>> {
        vec![Box::new(TeraLoader::new())]
    }

    fn templater(&self) -> Option<Box<dyn Templater>> {
        Some(Box::new(TeraTemplater::new()))
    }
}

impl TemplatePlugin {
    pub fn new() -> Self {
        TemplatePlugin {}
    }
}
