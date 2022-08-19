use clap::Args;

use crate::cli::Command;

use super::run;

#[derive(Debug, Args)]
pub struct RunCommand {
    /// Name of the bin target to run.
    #[clap(long = "bin", value_name = "NAME")]
    pub bin: Option<String>,

    /// Name of the example target to run.
    #[clap(long = "example", value_name = "NAME")]
    pub example: Option<String>,

    /// Build artifacts in release mode, with optimizations.
    #[clap(short = 'r', long = "release", default_value_t = false)]
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

    /// Run your game in the browser.
    #[clap(short = 'w', long = "wasm", default_value_t = false)]
    pub is_wasm: bool,
}

impl Command for RunCommand {
    fn exec(&self) {
        run(self);
    }
}
