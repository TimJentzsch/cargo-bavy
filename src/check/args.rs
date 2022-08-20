use crate::cargo::ArgBuilder;

use super::cli::CheckCommand;

impl From<&CheckCommand> for ArgBuilder {
    fn from(cmd: &CheckCommand) -> Self {
        let mut args = ArgBuilder::new();

        let wasm = Some("wasm32-unknown-unknown".to_string());

        // --wasm takes precedence over --target <TRIPLE>
        let target = if cmd.is_wasm { &wasm } else { &cmd.target };

        // Dynamic linking breaks release builds and doesn't work for WASM
        let features = if cmd.is_release || cmd.is_wasm {
            &None
        } else {
            &Some("bevy/dynamic")
        };

        args.add_opt_value("--package", &cmd.package)
            .add_flag("--workspace", cmd.is_workspace)
            .add_opt_value("--exclude", &cmd.exclude)
            .add_flag("--lib", cmd.is_lib)
            .add_flag("--locked", cmd.is_locked)
            .add_opt_value("--bin", &cmd.bin)
            .add_flag("--bins", cmd.is_bins)
            .add_opt_value("--example", &cmd.example)
            .add_flag("--examples", cmd.is_examples)
            .add_opt_value("--test", &cmd.test)
            .add_flag("--tests", cmd.is_tests)
            .add_opt_value("--bench", &cmd.bench)
            .add_flag("--benches", cmd.is_benches)
            .add_flag("--all-targets", cmd.is_all_targets)
            .add_flag("--release", cmd.is_release)
            .add_opt_value("--target", target)
            .add_opt_value("--target-dir", &cmd.target_dir)
            .add_opt_value("--manifest-path", &cmd.manifest_path)
            .add_opt_value("--features", features);

        args
    }
}
