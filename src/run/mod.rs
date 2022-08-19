use crate::cargo::{cargo_run, ArgBuilder};

use self::cli::RunCommand;

pub mod cli;

pub fn run(args: &RunCommand) {
    let mut cargo_args = ArgBuilder::new();

    // --bin <NAME>
    if let Some(name) = &args.bin {
        cargo_args.add_with_value("--bin", name);
    }

    // --example <NAME>
    if let Some(name) = &args.example {
        cargo_args.add_with_value("--example", name);
    }

    // --release
    if args.is_release {
        cargo_args.add("--release");
    }

    // --target <TRIPLE>
    if let Some(triple) = &args.target {
        cargo_args.add_with_value("--target", triple);
    }

    // --target-dir <DIRECTORY>
    if let Some(directory) = &args.target_dir {
        cargo_args.add_with_value("--target-dir", directory);
    }

    // --manifest-path <PATH>
    if let Some(path) = &args.manifest_path {
        cargo_args.add_with_value("--manifest-path", path);
    }

    // Enable dynamic linking if not in release mode
    if !args.is_release {
        cargo_args.add_with_value("--features", "bevy/dynamic");
    }

    cargo_run(cargo_args);
}
