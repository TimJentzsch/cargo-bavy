use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    process::Command,
};

use chrono::{Datelike, Local};
use dialoguer::MultiSelect;
use toml_edit::{value, Document};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Feature {
    FastCompileTimes,
    AssetHotReloading,
    WasmSupport,
    MitApacheLicenses,
}

impl Feature {
    /// A [`Vec`] containing all available features.
    fn all() -> Vec<Self> {
        vec![
            Feature::FastCompileTimes,
            Feature::AssetHotReloading,
            Feature::WasmSupport,
            Feature::MitApacheLicenses,
        ]
    }

    /// Determines if a feature should be enabled by default.
    fn enabled_by_default(&self) -> bool {
        true
    }
}

impl ToString for Feature {
    fn to_string(&self) -> String {
        match self {
            Feature::FastCompileTimes => "Fast compile times".to_string(),
            Feature::AssetHotReloading => "Hot reloading for assets".to_string(),
            Feature::WasmSupport => "WASM support".to_string(),
            Feature::MitApacheLicenses => {
                format!(
                    "MIT and Apache-2.0 licenses (Copyright (c) {})",
                    get_copyright_info()
                )
            }
        }
    }
}

fn main() {
    let folder_name = env::args()
        .nth(1)
        .expect("Please specify the name of the project.");

    let features = select_features();

    create_bevy_app(&folder_name, features);
}

fn select_features() -> Vec<Feature> {
    println!("Which features do you want?");
    println!("([Space] to select, [Enter] to confirm.)");

    let features = Feature::all();
    let selection = MultiSelect::new()
        .items(&features)
        .defaults(
            features
                .iter()
                .map(|feature| feature.enabled_by_default())
                .collect::<Vec<bool>>()
                .as_slice(),
        )
        .interact()
        .unwrap();

    features
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| selection.contains(idx))
        .map(|(_idx, feature)| feature)
        .collect()
}

fn create_bevy_app(folder_name: &str, features: Vec<Feature>) {
    create_cargo_app(folder_name);

    if features.contains(&Feature::MitApacheLicenses) {
        add_licenses(folder_name)
    }
}

fn create_cargo_app(folder_name: &str) {
    let output = Command::new("cargo")
        .arg("new")
        .arg(folder_name)
        .status()
        .expect("Failed to create the new project.")
        .success();

    if !output {
        panic!("Failed to create the new project.");
    }
}

fn add_licenses(folder_name: &str) {
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

    create_file_with_content(folder_name, "/LICENSE-MIT", mit_license).unwrap();
    create_file_with_content(folder_name, "/LICENSE-APACHE", apache_license).unwrap();

    set_cargo_toml_license(folder_name, "MIT OR Apache-2.0");
}

fn create_file_with_content<C>(folder_name: &str, path: &str, content: C) -> std::io::Result<()>
where
    C: Into<String>,
{
    let full_path = format!("{folder_name}/{path}");
    let mut file = File::create(full_path)?;
    file.write_all(content.into().as_bytes())?;

    Ok(())
}

fn get_copyright_info() -> String {
    let year = get_year();

    if let Some(author) = get_author() {
        format!("{year} {author}")
    } else {
        year
    }
}

/// Try to get the author from git.
///
/// `git config user.name`
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

fn get_cargo_toml(folder_name: &str) -> Document {
    let mut file =
        File::open(format!("{folder_name}/Cargo.toml")).expect("Failed to open `Cargo.toml`");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read `Cargo.toml`");

    content.parse().expect("Failed to parse `Cargo.toml`")
}

fn save_cargo_toml(folder_name: &str, cargo_toml: Document) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(format!("{folder_name}/Cargo.toml"))
        .expect("Failed to open `Cargo.toml`");

    let content = cargo_toml.to_string();
    file.write_all(content.as_bytes())
        .expect("Failed to write to `Cargo.toml`");
}

fn set_cargo_toml_license(folder_name: &str, license: &str) {
    let mut cargo_toml = get_cargo_toml(folder_name);
    cargo_toml["package"]["license"] = value(license);
    save_cargo_toml(folder_name, cargo_toml);
}
