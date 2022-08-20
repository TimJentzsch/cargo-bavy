mod args;
pub mod cli;

use crate::{
    cargo::{cargo_check, ArgBuilder},
    rustup::install_target_if_needed,
};

use self::cli::CheckCommand;

pub fn check(args: &CheckCommand) {
    if args.is_wasm {
        // Make sure that all tools are set up correctly

        // `wasm32-unknown-unknown` compilation target
        install_target_if_needed("wasm32-unknown-unknown", true, false)
            .expect("Installation of compilation target `wasm32-unknown-unknown` failed.");
    }

    let cargo_args = ArgBuilder::from(args);

    cargo_check(cargo_args);
}
