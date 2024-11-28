use crate::traits::finisher::Finisher;
use crate::traits::loader::Loader;
use crate::traits::picker::Picker;
use crate::traits::transformer::Transformer;
use crate::traits::Templater;

pub trait Plugin {
    fn pickers(&self) -> Vec<Box<dyn Picker>> {
        vec![]
    }

    fn loaders(&self) -> Vec<Box<dyn Loader>> {
        vec![]
    }

    fn transformers(&self) -> Vec<Box<dyn Transformer>> {
        vec![]
    }

    fn finishers(&self) -> Vec<Box<dyn Finisher>> {
        vec![]
    }

    fn templater(&self) -> Option<Box<dyn Templater>> {
        None
    }
}
