use toml_edit::{value, Item, Table};

use crate::{
    cargo::{get_cargo_toml, save_cargo_toml},
    new::context::Context,
};

/// Optimize dependencies in debug mode
pub fn optimize_dependencies(context: &mut Context) {
    context.extra_changes.push(Box::new(|context| {
        set_cargo_toml_dependency_optimizations(&context.folder_name);
    }))
}

fn set_cargo_toml_dependency_optimizations(folder_name: &str) {
    let mut cargo_toml = get_cargo_toml(folder_name).expect("Failed to get Cargo.toml");

    let mut profile = Table::new();
    profile.set_implicit(true);

    // Enable a small amount of optimization in debug mode
    // ```toml
    // [profile.dev]
    // opt-level = 1
    // ```
    let mut profile_dev = Table::new();
    profile_dev
        .decor_mut()
        .set_prefix("\n# Enable a small amount of optimization in debug mode\n");
    profile_dev.insert("opt-level", value(1));

    let mut profile_dev_package = Table::new();
    profile_dev_package.set_implicit(true);

    // Enable high optimizations for dependencies (incl. Bevy)
    // ```toml
    // [profile.dev.package."*"]
    // opt-level = 3
    // ```
    let mut profile_dev_package_all = Table::new();
    profile_dev_package_all
        .decor_mut()
        .set_prefix("\n# Enable high optimizations for dependencies (incl. Bevy)\n");
    profile_dev_package_all["opt-level"] = value(3);

    profile_dev_package.insert("*", Item::Table(profile_dev_package_all));
    profile_dev.insert("package", Item::Table(profile_dev_package));

    profile.insert("dev", Item::Table(profile_dev));
    cargo_toml.insert("profile", Item::Table(profile));

    save_cargo_toml(folder_name, cargo_toml);
}
