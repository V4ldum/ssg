#[cfg(feature = "markdown")]
pub mod markdown;
pub mod pickers;
mod pipeline;
#[cfg(feature = "pretty_url")]
pub mod pretty_url;
#[cfg(feature = "tailwind")]
pub mod tailwind;
#[cfg(feature = "template")]
pub mod template;
pub mod traits;

pub use pipeline::builder::SSGPipelineBuilder;
