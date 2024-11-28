use crate::info;
use crate::pipeline::SSGPipelineRunner;
use crate::traits::{Finisher, Loader, Picker, Plugin, Templater, Transformer};
use std::mem;

#[derive(Default)]
pub struct SSGPipelineBuilder {
    plugins: Vec<Box<dyn Plugin>>,
    pickers: Vec<Box<dyn Picker>>,
    loaders: Vec<Box<dyn Loader>>,
    transformers: Vec<Box<dyn Transformer>>,
    templater: Option<Box<dyn Templater>>,
    finishers: Vec<Box<dyn Finisher>>,
}

impl SSGPipelineBuilder {
    pub fn new() -> Self {
        SSGPipelineBuilder {
            ..Default::default()
        }
    }

    pub fn build(mut self) -> SSGPipelineRunner {
        info!("Generating website");
        info!();

        info!("Configuring plugins");
        let plugins = mem::take(&mut self.plugins);
        plugins.iter().for_each(|plugin| {
            // Pickers
            plugin
                .pickers()
                .into_iter()
                .for_each(|picker| self.add_picker_internal(picker));

            // Loaders
            plugin
                .loaders()
                .into_iter()
                .for_each(|loader| self.add_loader_internal(loader));

            // Transformers
            plugin
                .transformers()
                .into_iter()
                .for_each(|transformer| self.add_transformer_internal(transformer));

            // Templater
            plugin
                .templater()
                .into_iter()
                .for_each(|templater| self.add_templater_internal(templater));

            // Finishers
            plugin
                .finishers()
                .into_iter()
                .for_each(|finisher| self.add_finisher_internal(finisher));
        });
        info!();

        SSGPipelineRunner {
            pickers: self.pickers,
            loaders: self.loaders,
            transformers: self.transformers,
            templater: self.templater,
            finishers: self.finishers,
            ..Default::default()
        }
    }

    pub fn picker(mut self, picker: impl Picker + 'static) -> Self {
        self.add_picker_internal(Box::new(picker));

        self
    }

    fn add_picker_internal(&mut self, picker: Box<dyn Picker>) {
        self.pickers.push(picker);
    }

    pub fn loader(mut self, loader: impl Loader + 'static) -> Self {
        self.add_loader_internal(Box::new(loader));

        self
    }

    fn add_loader_internal(&mut self, loader: Box<dyn Loader>) {
        self.loaders.push(loader);
    }

    pub fn plugin(mut self, plugin: impl Plugin + 'static) -> Self {
        self.add_plugin_internal(Box::new(plugin));

        self
    }

    fn add_plugin_internal(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn transformer(mut self, transformer: impl Transformer + 'static) -> Self {
        self.add_transformer_internal(Box::new(transformer));

        self
    }

    fn add_transformer_internal(&mut self, transformer: Box<dyn Transformer>) {
        self.transformers.push(transformer);
    }

    pub fn templater(mut self, templater: impl Templater + 'static) -> Self {
        self.add_templater_internal(Box::new(templater));

        self
    }

    fn add_templater_internal(&mut self, templater: Box<dyn Templater>) {
        if let Some(templater) = &self.templater {
            panic!("A templater is already defined : {}", templater.name())
        }

        self.templater = Some(templater);
    }

    pub fn finisher(mut self, finisher: impl Finisher + 'static) -> Self {
        self.add_finisher_internal(Box::new(finisher));

        self
    }

    fn add_finisher_internal(&mut self, finisher: Box<dyn Finisher>) {
        self.finishers.push(finisher);
    }
}
