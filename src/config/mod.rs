// abap-tokenizer/src/config/mod.rs
pub(crate) mod tokenizer_config;
mod toml_loader;

pub use tokenizer_config::{TokenizerConfig, CompiledPatternConfig};
pub use toml_loader::load_toml_config;