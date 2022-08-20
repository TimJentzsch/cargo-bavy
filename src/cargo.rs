use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    process::{exit, Command},
};

use anyhow::Result;
use toml_edit::Document;

pub struct ArgBuilder(Vec<String>);

impl ArgBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add an argument.
    ///
    /// Can be a `&str` or a `String`.
    /// If it contains whitespace, it is split into multiple args.
    pub fn add<A>(&mut self, arg: A) -> &mut Self
    where
        A: Into<String>,
    {
        let arg: String = arg.into();

        for part in arg.split_ascii_whitespace() {
            self.0.push(part.to_string());
        }

        self
    }

    /// Add an argument with a value.
    pub fn add_with_value<A, V>(&mut self, arg: A, value: V) -> &mut Self
    where
        A: Into<String>,
        V: Into<String>,
    {
        self.add(arg);
        self.add(value);

        self
    }

    /// Add a boolean flag with the given name, if `value` is `true`.
    ///
    /// Example: `--release`
    pub fn add_flag<N>(&mut self, name: N, value: bool) -> &mut Self
    where
        N: Into<String>,
    {
        if value {
            self.add(name);
        }

        self
    }

    /// Add an argument with an optional value.
    ///
    /// Example: `--bin <NAME>`
    pub fn add_opt_value<N>(&mut self, name: N, value: Option<String>) -> &mut Self
    where
        N: Into<String>,
    {
        if let Some(value) = value {
            self.add_with_value(name, value);
        }

        self
    }
}

/// Run `cargo run` with the given arguments.
///
/// Exits the program if `cargo run` failed.
pub fn cargo_run(args: ArgBuilder) {
    let status = Command::new("cargo")
        .arg("run")
        .args(args.0)
        .status()
        .expect("Failed to run `cargo run`.");

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

/// Run `cargo build` with the given arguments.
///
/// Exits the program if `cargo run` failed.
pub fn cargo_build(args: ArgBuilder) {
    let status = Command::new("cargo")
        .arg("build")
        .args(args.0)
        .status()
        .expect("Failed to run `cargo build`.");

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

pub fn get_cargo_toml(folder_name: &str) -> Result<Document> {
    let mut file = File::open(format!("{folder_name}/Cargo.toml"))?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content.parse()?)
}

pub fn save_cargo_toml(folder_name: &str, cargo_toml: Document) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(format!("{folder_name}/Cargo.toml"))
        .expect("Failed to open `Cargo.toml`");

    let content = cargo_toml.to_string();
    file.write_all(content.as_bytes())
        .expect("Failed to write to `Cargo.toml`");
}
