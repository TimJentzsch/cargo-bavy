use crate::new::{
    compile_features::CompileFeature,
    context::{Context, CreateFile},
};

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
    let mut release_workflow = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/.github/workflows/release.yaml"
    ))
    .replace("{{{project_name}}}", &context.folder_name);

    // Include a job to publish WASM if needed
    if context
        .compile_features
        .contains(&CompileFeature::WasmTarget)
    {
        let wasm_job = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/.github/workflows/release-wasm.txt"
        ));

        release_workflow = release_workflow.replace("{{{wasm}}}", wasm_job);
    } else {
        release_workflow = release_workflow.replace("{{{wasm}}}", "");
    }

    // Clean up file ending
    release_workflow = release_workflow.trim_end().to_string() + "\n";

    context.create_files.push(CreateFile::new(
        "/.github/workflows/release.yaml",
        release_workflow,
    ));
}
