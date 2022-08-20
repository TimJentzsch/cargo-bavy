use clap::{ArgAction, Args};

use crate::cli::Command;

use super::build;

#[derive(Debug, Args)]
pub struct BuildCommand {
    /// Package to build (see `cargo help pkgid`).
    #[clap(short = 'p', long = "package", value_name = "SPEC")]
    pub package: Option<String>,

    /// Build all packages in the workspace.
    #[clap(long = "workspace", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_workspace: bool,

    ///  Exclude packages from the build.
    #[clap(long = "exclude", value_name = "SPEC")]
    pub exclude: Option<String>,

    /// Build only this package's library.
    #[clap(long = "lib", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_lib: bool,

    /// Require Cargo.lock is up to date.
    #[clap(long = "locked", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_locked: bool,

    /// Build only the specified binary.
    #[clap(long = "bin", value_name = "NAME")]
    pub bin: Option<String>,

    /// Build all binaries.
    #[clap(long = "bins", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_bins: bool,

    /// Build only the specified example.
    #[clap(long = "example", value_name = "NAME")]
    pub example: Option<String>,

    /// Build all examples.
    #[clap(long = "examples", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_examples: bool,

    /// Build only the specified test target.
    #[clap(long = "test", value_name = "NAME")]
    pub test: Option<String>,

    /// Build all tests.
    #[clap(long = "tests", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_tests: bool,

    /// Build only the specified bench target.
    #[clap(long = "bench", value_name = "NAME")]
    pub bench: Option<String>,

    /// Build all benches.
    #[clap(long = "benches", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_benches: bool,

    /// Build all benches.
    #[clap(long = "all-targets", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_all_targets: bool,

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
