use crate::traits::Finisher;
use crate::{err, progress};
use std::process::Command;

#[derive(Default)]
pub struct TailwindFinisher {}

impl Finisher for TailwindFinisher {
    fn finish(&self) {
        // todo dynamic
        let input = "source/styles/tailwind.css";
        let output = "build/styles/tailwind.css";
        let tailwind_path = "./tailwindcss";
        let config_file = "tailwind.config.js";

        progress!("Generating Tailwind CSS");

        let _ignored = Command::new(tailwind_path)
            .args([
                //
                "-i",
                input,
                "-o",
                output,
                "-c",
                config_file,
                #[cfg(not(debug_assertions))]
                "--minify",
            ])
            .output()
            .map_err(|err| {
                err!("Failed to generate TailwindCSS : {err}");
            });
    }
}

impl TailwindFinisher {
    pub fn new() -> Self {
        TailwindFinisher::default()
    }
}
