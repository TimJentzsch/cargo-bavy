use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};

use dialoguer::MultiSelect;

use super::feature::Feature;

pub fn create_file_with_content<F, P, C>(folder_name: F, path: P, content: C) -> std::io::Result<()>
where
    F: Into<String>,
    P: Into<String>,
    C: Into<String>,
{
    let full_path = PathBuf::from(format!("{}/{}", folder_name.into(), path.into()));

    // Create the directory if it doesn't exist
    if let Some(dir_path) = full_path.parent() {
        create_dir_all(dir_path).expect("Failed to create parent folder.");
    }

    // Create the file
    let mut file = File::create(full_path)?;
    // Write the content in the file
    file.write_all(content.into().as_bytes())?;

    Ok(())
}

pub fn save_main_rs(folder_name: &str, main_rs: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(format!("{folder_name}/src/main.rs"))
        .expect("Failed to open `src/main/.rs`");

    file.write_all(main_rs.as_bytes())
        .expect("Failed to write to `src/main/.rs`");
}

pub fn add_dependency(folder_name: &str, name: &str, features: Vec<String>) {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(folder_name).arg("add").arg(name);

    if !features.is_empty() {
        cmd.args(&features);
    }

    let output = cmd
        .output()
        .unwrap_or_else(|_| panic!("Failed to add dependency {name} with features {features:?}"));

    if !output.status.success() {
        panic!("Failed to add dependency {name} with features {features:?}");
    }
}

pub fn select_features<F>(prompt: &str) -> Vec<F>
where
    F: Feature,
{
    println!("{prompt}");

    let features = F::all();
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

/// Run `cargo fmt` on the new project.
pub fn run_cargo_fmt(folder_name: &str) {
    let output = Command::new("cargo")
        .arg("fmt")
        .current_dir(folder_name)
        .output()
        .expect("Failed to format the project.");

    if !output.status.success() {
        panic!("FAiled to format the project.");
    }
}
