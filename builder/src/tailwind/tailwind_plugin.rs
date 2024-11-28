use crate::tailwind::TailwindFinisher;
use crate::traits::{Finisher, Plugin};

#[derive(Default)]
pub struct TailwindPlugin {}

impl Plugin for TailwindPlugin {
    fn finishers(&self) -> Vec<Box<dyn Finisher>> {
        vec![Box::new(TailwindFinisher::new())]
    }
}

impl TailwindPlugin {
    pub fn new() -> Self {
        TailwindPlugin::default()
    }
}
