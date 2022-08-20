mod bevy_features;
pub mod cli;
mod compile_features;
pub mod context;
mod feature;
mod project_features;
mod utils;

use std::process::Command;

use dialoguer::console::style;

use crate::files::create_file_with_content;

use self::{
    bevy_features::{select_bevy_features, BevyFeature},
    compile_features::{register_compile_features, select_compile_features},
    context::{AddDependency, Context},
    project_features::{register_project_features, select_project_features},
    utils::{add_dependency, run_cargo_fmt, save_main_rs},
};

pub fn new(folder_name: &str) {
    println!(
        "{} Bevy app `{}`...\n",
        style("Creating").green().bold(),
        style(folder_name).cyan()
    );
    println!(
        "- Use {} to select/unselect options",
        style("[Space]").blue()
    );
    println!("- Use {} to confirm selection\n", style("[Enter]").blue());

    let mut context = Context::new(folder_name);
    context.bevy_features = select_bevy_features();
    context.compile_features = select_compile_features();
    context.project_features = select_project_features();

    create_bevy_app(context);
}

fn create_bevy_app(mut context: Context) {
    let folder_name = context.folder_name.clone();

    create_cargo_app(&mut context);

    register_compile_features(&mut context);
    register_project_features(&mut context);

    create_files(&mut context);
    add_dependencies(&mut context);
    adjust_main_file(&mut context);
    apply_extra_changes(context);

    println!(
        "\n{} Bevy app `{}`!",
        style("Created").green().bold(),
        style(&folder_name).cyan()
    );
    println!("\nNext steps:");
    println!("$ cd {folder_name}");
    println!("$ cargo bavy run");
}

fn create_cargo_app(context: &mut Context) {
    let output = Command::new("cargo")
        .arg("new")
        .arg(&context.folder_name)
        .output()
        .expect("Failed to create the new project.");

    if !output.status.success() {
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
    context.add_dependencies.push(AddDependency {
        name: "bevy".to_string(),
        features: vec![],
    });

    for dependency in &context.add_dependencies {
        add_dependency(
            &context.folder_name,
            &dependency.name,
            dependency.features.clone(),
        );
    }
}

fn adjust_main_file(context: &mut Context) {
    let mut imports = vec!["use bevy::prelude::*;"];
    let mut builder_methods = vec![];

    if context
        .bevy_features
        .contains(&BevyFeature::AssetHotReloading)
    {
        imports.push("use bevy::asset::AssetServerSettings;");

        builder_methods.push(
            "// Enable hot reloading
            .insert_resource(AssetServerSettings {
                watch_for_changes: true,
                ..default()
            })",
        )
    }

    builder_methods.push(".add_plugins(DefaultPlugins)");
    builder_methods.push(".run()");

    // Note: The double braces are necessary to not collide with the format syntax
    let main_rs = format!(
        "{}

        fn main() {{
            App::new()
                {};
        }}
    ",
        imports.join("\n"),
        builder_methods.join("\n")
    );

    save_main_rs(&context.folder_name, main_rs);
    // Run `cargo fmt` to make sure that everything is tidy
    run_cargo_fmt(&context.folder_name);
}

fn apply_extra_changes(mut context: Context) {
    let extra_changes = context.extra_changes;
    context.extra_changes = Vec::new();

    for change in &extra_changes {
        change(&context);
    }
}
