//! Utility module to extract information from the environment.

use std::process::Command;

use anyhow::{anyhow, Result};
use chrono::{Datelike, Local};
use toml_edit::{Item, Value};

use crate::cargo::get_cargo_toml;

/// Try to get the author from git.
///
/// This uses `git config user.name`.
pub fn get_author() -> Option<String> {
    // Run `git config user.name`
    Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        // Remove surrounding whitespace
        .map(|author| author.trim().to_string())
}

/// Get the current year.
pub fn get_year() -> String {
    Local::now().date().year().to_string()
}

/// Tries to get the crate name from `Cargo.toml`.
pub fn get_crate_name() -> Result<String> {
    let cargo_toml = get_cargo_toml("./")?;

    if let Item::Value(Value::String(name)) = &cargo_toml["package"]["name"] {
        Ok(name.value().clone())
    } else {
        Err(anyhow!("No package name defined in Cargo.toml"))
    }
}
