use toml_edit::{value, Item, Table};

use crate::new::{
    context::Context,
    utils::{get_cargo_toml, save_cargo_toml},
};

/// Optimize dependencies in debug mode
pub fn optimize_dependencies(context: &mut Context) {
    context.extra_changes.push(Box::new(|context| {
        set_cargo_toml_dependency_optimizations(&context.folder_name);
    }))
}

fn set_cargo_toml_dependency_optimizations(folder_name: &str) {
    let mut cargo_toml = get_cargo_toml(folder_name);

    // Enable a small amount of optimization in debug mode
    let mut dev_profile = Table::new();
    dev_profile.insert("opt_level", value(1));
    cargo_toml.insert("profile.dev", Item::Table(dev_profile));

    // Enable high optimizations for dependencies (incl. Bevy)
    let mut dependency_dev_profile = Table::new();
    dependency_dev_profile.insert("opt_level", value(3));
    cargo_toml.insert(
        r#"profile.dev.package."*""#,
        Item::Table(dependency_dev_profile),
    );

    save_cargo_toml(folder_name, cargo_toml);
}
