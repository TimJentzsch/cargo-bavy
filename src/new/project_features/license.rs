use toml_edit::value;

use crate::{
    cargo::{get_cargo_toml, save_cargo_toml},
    env::{get_author, get_year},
    new::context::{Context, CreateFile},
};

pub fn add_licenses(context: &mut Context) {
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

    context
        .create_files
        .push(CreateFile::new("/LICENSE-MIT", mit_license));
    context
        .create_files
        .push(CreateFile::new("/LICENSE-APACHE", apache_license));

    context.extra_changes.push(Box::new(|context| {
        set_cargo_toml_license(&context.folder_name, "MIT OR Apache-2.0");
    }))
}

/// Set the `package.license` field in `Cargo.toml`
fn set_cargo_toml_license(folder_name: &str, license: &str) {
    let mut cargo_toml = get_cargo_toml(folder_name).expect("Failed to get Cargo.toml");
    cargo_toml["package"]["license"] = value(license);
    save_cargo_toml(folder_name, cargo_toml);
}

/// Get the current copyright info in the format `{year} {author}`.
///
/// The author is determined via Git.
pub fn get_copyright_info() -> String {
    let year = get_year();

    if let Some(author) = get_author() {
        format!("{year} {author}")
    } else {
        year
    }
}
