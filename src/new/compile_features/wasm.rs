use crate::new::context::{Context, CreateFile};

pub fn add_wasm(context: &mut Context) {
    let wasm_index = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/wasm/index.html"
    ));

    let wasm_gitignore = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/wasm/gitignore.txt"
    ));

    context
        .create_files
        .push(CreateFile::new("/wasm/index.html", wasm_index));
    context
        .create_files
        .push(CreateFile::new("/wasm/.gitignore", wasm_gitignore));
}
