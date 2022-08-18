use std::{env, process::Command};

fn main() {
    let folder_name = env::args()
        .nth(1)
        .expect("Please specify the name of the project.");
}

fn create_project(folder_name: String) {
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
