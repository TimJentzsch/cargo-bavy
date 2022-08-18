use crate::new::context::{Context, CreateFile};

pub fn add_nightly_toolchain(context: &mut Context) {
    let toolchain_toml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/nightly/rust-toolchain.toml"
    ));

    context
        .create_files
        .push(CreateFile::new("/rust-toolchain.toml", toolchain_toml));
}
