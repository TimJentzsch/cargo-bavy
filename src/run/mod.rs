use std::process::{exit, Command};

pub mod cli;

pub fn run() {
    let status = Command::new("cargo")
        .arg("run")
        .arg("--features")
        .arg("bevy/dynamic")
        .status()
        .expect("Failed to run `cargo run`.");

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}
