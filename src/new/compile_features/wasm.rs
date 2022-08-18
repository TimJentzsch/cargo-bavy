use crate::new::context::{Context, CreateFile};

pub fn add_wasm(context: &mut Context) {
    let wasm_index = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/wasm/index.html"
    ));

    context
        .create_files
        .push(CreateFile::new("/wasm/index.html", wasm_index));
}
