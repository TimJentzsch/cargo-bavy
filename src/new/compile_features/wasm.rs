use crate::new::utils::create_file_with_content;

pub fn add_wasm(folder_name: &str) {
    let wasm_index = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/wasm/index.html"
    ));

    create_file_with_content(folder_name, "/wasm/index.html", wasm_index).unwrap();
}
