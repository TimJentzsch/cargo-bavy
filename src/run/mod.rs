use std::process::{exit, Command};

use self::cli::RunCommand;

pub mod cli;

struct CargoArgs(Vec<String>);

impl CargoArgs {
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Add an argument.
    ///
    /// Can be a `&str` or a `String`.
    /// If it contains whitespace, it is split into multiple args.
    fn add<A>(&mut self, arg: A)
    where
        A: Into<String>,
    {
        let arg: String = arg.into();

        for part in arg.split_ascii_whitespace() {
            self.0.push(part.to_string());
        }
    }

    /// Add an argument with a value.
    fn add_with_value<A, V>(&mut self, arg: A, value: V)
    where
        A: Into<String>,
        V: Into<String>,
    {
        self.add(arg);
        self.add(value);
    }
}

pub fn run(args: &RunCommand) {
    let mut cargo_args = CargoArgs::new();

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

    let status = Command::new("cargo")
        .arg("run")
        .args(cargo_args.0)
        .status()
        .expect("Failed to run `cargo run`.");

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}
