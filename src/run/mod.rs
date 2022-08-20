mod args;
pub mod cli;

use dialoguer::console::{style, Style};

use crate::{
    cargo::{cargo_build, cargo_run, ArgBuilder},
    files::replace_folder,
    http_server::launch_game,
    rustup::install_target_if_needed,
    wasm_bindgen::{bundle_to_web, create_wasm_folder_if_needed, install_wasm_bindgen_if_needed},
};

use self::cli::RunCommand;

pub fn run(args: &RunCommand) {
    let info_style = Style::new().magenta().dim();

    if args.is_wasm {
        // Make sure that all tools are set up correctly

        // `wasm32-unknown-unknown` compilation target
        install_target_if_needed("wasm32-unknown-unknown", true, false)
            .expect("Installation of compilation target `wasm32-unknown-unknown` failed.");
        // `wasm-bindgen-cli` for bundling
        install_wasm_bindgen_if_needed(true, false)
            .expect("Installation of `wasm-bindgen-cli` failed.");
        // `wasm/` target folder
        create_wasm_folder_if_needed(true).expect("Creation of `wasm/` folder failed.");
    }

    let cargo_args = ArgBuilder::from(args);

    if args.is_wasm {
        println!("{}", info_style.apply_to("Building for WASM..."));
        cargo_build(cargo_args);

        println!("{}", info_style.apply_to("Bundling for the web..."));
        bundle_to_web(args.is_release).expect("Failed to bundle for the web");

        println!("{}", info_style.apply_to("Updating assets..."));
        replace_folder("assets", "wasm").expect("Failed to update asset folder");

        println!("{}", info_style.apply_to("Serving on localhost..."));
        println!(
            "Open your game at <{}>.",
            style("http://127.0.0.1:4000").green()
        );
        launch_game().expect("Failed to launch game");
    } else {
        cargo_run(cargo_args);
    }
}
