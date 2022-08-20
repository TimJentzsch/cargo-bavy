use std::{
    path::Path,
    process::{exit, Command},
};

use anyhow::{anyhow, Result};
use dialoguer::Confirm;

use crate::{env::get_crate_name, files::create_file_with_content};

/// Try to determine if `wasm-bindgen-cli` is installed.
///
/// Returns `true` if it is installed and `false` otherwise.
///
/// This works by trying out the `wasm-bindgen --version` command.
pub fn is_wasm_bindgen_installed() -> bool {
    let output = Command::new("wasm-bindgen").arg("--version").output();

    if let Ok(output) = output {
        output.status.success()
    } else {
        false
    }
}

/// Check if `wasm-bindgen-cli` is installed already and install it if it's not.
///
/// If `ask_user` is set to `true`, it will first prompt the user and abort otherwise.
/// If `hidden` is set to `true`, the user won't be able to see the output.
pub fn install_wasm_bindgen_if_needed(ask_user: bool, hidden: bool) -> Result<()> {
    if is_wasm_bindgen_installed() {
        return Ok(());
    }

    // Abort if the user doesn't want to install it
    if ask_user
        && !Confirm::new()
            .with_prompt("`wasm-bindgen-cli` is missing, should I install it for you?")
            .interact()
            .unwrap()
    {
        exit(1);
    }

    let mut cmd = Command::new("cargo");
    cmd.arg("install").arg("wasm-bindgen-cli");

    let status = if hidden {
        cmd.output()?.status
    } else {
        cmd.status()?
    };

    if !status.success() {
        Err(anyhow!("Failed to install `wasm-bindgen-cli`."))
    } else {
        Ok(())
    }
}

/// Run `wasm-bindgen` to bundle the binary for the web.
///
/// Requires a build artifact from `cargo build`.
pub fn bundle_to_web(is_release: bool) -> Result<()> {
    let name = get_crate_name()?;

    let artifact_folder = if is_release { "release" } else { "debug" };

    let status = Command::new("wasm-bindgen")
        .args([
            "--no-typescript",
            "--out-name",
            "bevy_game",
            "--out-dir",
            "wasm",
            "--target",
            "web",
        ])
        .arg(format!(
            "target/wasm32-unknown-unknown/{artifact_folder}/{name}.wasm"
        ))
        .status()?;

    if !status.success() {
        Err(anyhow!("Failed to bundle project for the web."))
    } else {
        Ok(())
    }
}

/// Determine if the `/wasm/` folder exists already.
pub fn does_wasm_folder_exist() -> bool {
    Path::new("wasm").exists()
}

/// Create the `wasm/` folder for the WASM build artifacts if needed.
pub fn create_wasm_folder_if_needed(ask_user: bool) -> Result<()> {
    if does_wasm_folder_exist() {
        return Ok(());
    }

    // Abort if the user doesn't want to create it
    if ask_user
        && !Confirm::new()
            .with_prompt(
                "The `wasm/` folder for the WASM build is missing, should I create it for you?",
            )
            .interact()
            .unwrap()
    {
        exit(1);
    }

    // Create the folder and necessary files
    let wasm_index = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/wasm/index.html"
    ));

    let wasm_gitignore = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/wasm/gitignore.txt"
    ));

    create_file_with_content(".", "wasm/index.html", wasm_index)?;
    create_file_with_content(".", "wasm/.gitignore", wasm_gitignore)?;

    Ok(())
}
