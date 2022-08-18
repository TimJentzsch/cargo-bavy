use crate::new::context::{Context, CreateFile};

pub fn add_ci_workflow(context: &mut Context) {
    let ci_workflow = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/.github/workflows/ci.yaml"
    ));

    context
        .create_files
        .push(CreateFile::new("/.github/workflows/ci.yaml", ci_workflow));
}

pub fn add_release_workflow(context: &mut Context) {
    // TODO: Only include WASM workflow if the WASM feature is enabled
    let release_workflow = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/.github/workflows/release.yaml"
    ))
    .replace("{{{project_name}}}", &context.folder_name);

    context.create_files.push(CreateFile::new(
        "/.github/workflows/release.yaml",
        release_workflow,
    ));
}
