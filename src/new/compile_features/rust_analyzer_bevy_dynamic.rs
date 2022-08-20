use crate::new::context::{Context, CreateFile};

/// Enables the feature `bevy/dynamic` for Rust Analyzer.
///
/// This gives faster compiles and prevents Rust Analyzer checks from clashing
/// with `cargo bavy` commands.
pub fn enable_bevy_dynamic_for_rust_analyzer(context: &mut Context) {
    let vscode_settings = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/.vscode/settings.json"
    ));

    context
        .create_files
        .push(CreateFile::new("/.vscode/settings.json", vscode_settings));
}
