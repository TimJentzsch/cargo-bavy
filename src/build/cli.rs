use clap::{ArgAction, Args};

use crate::cli::Command;

use super::build;

#[derive(Debug, Args)]
pub struct BuildCommand {
    /// Build only the specified binary.
    #[clap(long = "bin", value_name = "NAME")]
    pub bin: Option<String>,

    /// Build only the specified example.
    #[clap(long = "example", value_name = "NAME")]
    pub example: Option<String>,

    /// Build artifacts in release mode, with optimizations.
    #[clap(short = 'r', long = "release", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_release: bool,

    /// Build for the target triple.
    #[clap(long = "target", value_name = "TRIPLE")]
    pub target: Option<String>,

    /// Directory for all generated artifacts.
    #[clap(long = "target-dir", value_name = "DIRECTORY")]
    pub target_dir: Option<String>,

    /// Path to Cargo.toml.
    #[clap(long = "manifest-path", value_name = "PATH")]
    pub manifest_path: Option<String>,

    /// Build the game for the browser.
    #[clap(short = 'w', long = "wasm", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_wasm: bool,
}

impl Command for BuildCommand {
    fn exec(&self) {
        build(self);
    }
}
