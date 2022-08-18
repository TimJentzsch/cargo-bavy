use crate::new::context::{Context, CreateFile};

pub fn add_fast_linker(context: &mut Context) {
    let config_toml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/.cargo/config.toml"
    ));

    context
        .create_files
        .push(CreateFile::new("/.cargo/config.toml", config_toml));
}
