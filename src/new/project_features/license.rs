use std::process::Command;

use chrono::{Datelike, Local};
use toml_edit::value;

use crate::new::{
    context::{Context, CreateFile},
    utils::{get_cargo_toml, save_cargo_toml},
};

pub fn add_licenses(context: &mut Context) {
    let mit_license = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/licenses/LICENSE-MIT"
    ))
    // MIT requires copyright info
    .replace("{{copyright}}", &get_copyright_info());

    let apache_license = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/licenses/LICENSE-APACHE"
    ));

    context
        .create_files
        .push(CreateFile::new("/LICENSE-MIT", mit_license));
    context
        .create_files
        .push(CreateFile::new("/LICENSE-APACHE", apache_license));

    context.extra_changes.push(Box::new(|context| {
        set_cargo_toml_license(&context.folder_name, "MIT OR Apache-2.0");
    }))
}

/// Set the `package.license` field in `Cargo.toml`
fn set_cargo_toml_license(folder_name: &str, license: &str) {
    let mut cargo_toml = get_cargo_toml(folder_name);
    cargo_toml["package"]["license"] = value(license);
    save_cargo_toml(folder_name, cargo_toml);
}

/// Get the current copyright info in the format `{year} {author}`.
///
/// The author is determined via Git.
pub fn get_copyright_info() -> String {
    let year = get_year();

    if let Some(author) = get_author() {
        format!("{year} {author}")
    } else {
        year
    }
}

/// Try to get the author from git.
///
/// This uses `git config user.name`.
fn get_author() -> Option<String> {
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
fn get_year() -> String {
    Local::now().date().year().to_string()
}
