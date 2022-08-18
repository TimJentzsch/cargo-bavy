use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    process::Command,
};

use toml_edit::Document;

pub fn create_file_with_content<C>(folder_name: &str, path: &str, content: C) -> std::io::Result<()>
where
    C: Into<String>,
{
    let full_path = format!("{folder_name}/{path}");
    let mut file = File::create(full_path)?;
    file.write_all(content.into().as_bytes())?;

    Ok(())
}

pub fn get_cargo_toml(folder_name: &str) -> Document {
    let mut file =
        File::open(format!("{folder_name}/Cargo.toml")).expect("Failed to open `Cargo.toml`");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read `Cargo.toml`");

    content.parse().expect("Failed to parse `Cargo.toml`")
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

pub fn add_dependency(folder_name: &str, dependency: &str, features: Vec<&str>) {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(folder_name).arg("add").arg(dependency);

    if !features.is_empty() {
        cmd.args(&features);
    }

    let status = cmd.status().unwrap_or_else(|_| {
        panic!("Failed to add dependency {dependency} with features {features:?}")
    });

    if !status.success() {
        panic!("Failed to add dependency {dependency} with features {features:?}");
    }
}
