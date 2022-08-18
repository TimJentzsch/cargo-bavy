mod bevy_features;
pub mod cli;
mod compile_features;
pub mod context;
mod feature;
mod project_features;
mod utils;

use std::process::Command;

use self::{
    bevy_features::select_bevy_features,
    compile_features::{select_compile_features, wasm::add_wasm, CompileFeature},
    context::Context,
    project_features::{license::add_licenses, select_project_features, ProjectFeature},
    utils::add_dependency,
};

pub fn new(folder_name: &str) {
    let mut context = Context::new(folder_name);
    context.bevy_features = select_bevy_features();
    context.compile_features = select_compile_features();
    context.project_features = select_project_features();

    create_bevy_app(&mut context);
}

fn create_bevy_app(context: &mut Context) {
    create_cargo_app(context);

    if context
        .project_features
        .contains(&ProjectFeature::MitApacheLicenses)
    {
        add_licenses(&context.folder_name);
    }

    if context
        .compile_features
        .contains(&CompileFeature::WasmTarget)
    {
        add_wasm(&context.folder_name);
    }

    add_dependencies(context);
}

fn create_cargo_app(context: &mut Context) {
    let output = Command::new("cargo")
        .arg("new")
        .arg(&context.folder_name)
        .status()
        .expect("Failed to create the new project.")
        .success();

    if !output {
        panic!("Failed to create the new project.");
    }
}

fn add_dependencies(context: &mut Context) {
    for dependency in &context.add_dependencies {
        add_dependency(
            &context.folder_name,
            &dependency.name,
            dependency.features.clone(),
        );
    }
}
