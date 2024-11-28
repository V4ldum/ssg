use crate::markdown::MarkdownLoader;
use crate::markdown::MarkdownTransformer;
use crate::pickers::file_picker::FileFilter::Extension;
use crate::pickers::FilePicker;
use crate::traits::Loader;
use crate::traits::Picker;
use crate::traits::Plugin;
use crate::traits::Transformer;

#[derive(Default)]
pub struct MarkdownPlugin {}

impl Plugin for MarkdownPlugin {
    fn pickers(&self) -> Vec<Box<dyn Picker>> {
        vec![Box::new(FilePicker::new(Extension("md")))]
    }

    fn loaders(&self) -> Vec<Box<dyn Loader>> {
        vec![Box::new(MarkdownLoader::new())]
    }

    fn transformers(&self) -> Vec<Box<dyn Transformer>> {
        vec![Box::new(MarkdownTransformer::new())]
    }
}

impl MarkdownPlugin {
    pub fn new() -> Self {
        MarkdownPlugin::default()
    }
}
