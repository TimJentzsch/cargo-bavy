mod bevy_features;
pub mod cli;
mod compile_features;
mod feature;
mod project_features;
mod utils;

use std::process::Command;

use self::{
    bevy_features::{select_bevy_features, BevyFeature},
    compile_features::{select_compile_features, wasm::add_wasm, CompileFeature},
    project_features::{license::add_licenses, select_project_features, ProjectFeature},
    utils::add_dependency,
};

pub fn new(folder_name: &str) {
    let bevy_features = select_bevy_features();
    let compile_features = select_compile_features();
    let project_features = select_project_features();

    create_bevy_app(
        folder_name,
        bevy_features,
        compile_features,
        project_features,
    );
}

fn create_bevy_app(
    folder_name: &str,
    bevy_features: Vec<BevyFeature>,
    compile_features: Vec<CompileFeature>,
    project_features: Vec<ProjectFeature>,
) {
    create_cargo_app(folder_name);

    if project_features.contains(&ProjectFeature::MitApacheLicenses) {
        add_licenses(folder_name);
    }

    if compile_features.contains(&CompileFeature::WasmTarget) {
        add_wasm(folder_name);
    }

    add_dependencies(folder_name);
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

fn add_dependencies(folder_name: &str) {
    add_dependency(folder_name, "bevy", vec![]);
}
