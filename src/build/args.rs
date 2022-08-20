use crate::cargo::ArgBuilder;

use super::cli::BuildCommand;

impl From<&BuildCommand> for ArgBuilder {
    fn from(cmd: &BuildCommand) -> Self {
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

        args.add_opt_value("--bin", &cmd.bin)
            .add_opt_value("--example", &cmd.example)
            .add_flag("--release", cmd.is_release)
            .add_opt_value("--target", target)
            .add_opt_value("--target-dir", &cmd.target_dir)
            .add_opt_value("--manifest-path", &cmd.manifest_path)
            .add_opt_value("--features", features);

        args
    }
}
