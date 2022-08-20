use crate::{
    new::context::{Context, CreateFile},
    rustup::install_target_if_needed,
    wasm_bindgen::install_wasm_bindgen_if_needed,
};

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

    context.extra_changes.push(Box::new(|_| {
        install_target_if_needed("wasm32-unknown-unknown", false, true)
            .expect("Failed to install `wasm32-unknown-unknown` target.");
    }));
    context.extra_changes.push(Box::new(|_| {
        install_wasm_bindgen_if_needed(false, true).expect("Failed to install `wasm-bindgen-cli`.");
    }));
}
