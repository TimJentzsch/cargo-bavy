use std::process::{exit, Command};

use dialoguer::Confirm;

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

/// Checks if `wasm-bindgen-cli` is installed already and installs it if it's not.
///
/// If `ask_user` is set to `true`, it will first prompt the user and abort otherwise.
/// If `hidden` is set to `true`, the user won't be able to see the output.
pub fn install_wasm_bindgen_if_needed(ask_user: bool, hidden: bool) {
    if is_wasm_bindgen_installed() {
        return;
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
        cmd.output().unwrap().status
    } else {
        cmd.status().unwrap()
    };

    if !status.success() {
        panic!("Installation of `wasm-bindgen-cli` failed!");
    }
}
