use clap::Parser;
use cli::{CargoCommand, Command};

mod build;
pub(crate) mod cargo;
mod cli;
pub(crate) mod env;
pub(crate) mod files;
pub(crate) mod http_server;
mod new;
mod run;
pub(crate) mod rustup;
pub(crate) mod wasm_bindgen;

fn main() {
    let cmd = CargoCommand::parse();
    cmd.exec();
}
