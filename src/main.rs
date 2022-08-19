use clap::Parser;
use cli::{CargoCommand, Command};

pub(crate) mod cargo;
mod cli;
mod new;
mod run;
pub(crate) mod tools;

fn main() {
    let cmd = CargoCommand::parse();
    cmd.exec();
}
