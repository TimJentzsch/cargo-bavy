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
    compile_features::{register_compile_features, select_compile_features},
    context::Context,
    project_features::{register_project_features, select_project_features},
    utils::{add_dependency, create_file_with_content},
};

pub fn new(folder_name: &str) {
    let mut context = Context::new(folder_name);
    context.bevy_features = select_bevy_features();
    context.compile_features = select_compile_features();
    context.project_features = select_project_features();

    create_bevy_app(context);
}

fn create_bevy_app(mut context: Context) {
    create_cargo_app(&mut context);

    register_compile_features(&mut context);
    register_project_features(&mut context);

    create_files(&mut context);
    add_dependencies(&mut context);
    apply_extra_changes(context)
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

fn create_files(context: &mut Context) {
    for file in &context.create_files {
        create_file_with_content(&context.folder_name, &file.path, file.content.clone())
            .expect("Failed to create file");
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

fn apply_extra_changes(mut context: Context) {
    let extra_changes = context.extra_changes;
    context.extra_changes = Vec::new();

    for change in &extra_changes {
        change(&context);
    }
}
