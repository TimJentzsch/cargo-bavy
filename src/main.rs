use clap::Parser;
use cli::{CargoCommand, Command};

pub(crate) mod cargo;
mod cli;
mod new;
mod run;

fn main() {
    let cmd = CargoCommand::parse();
    cmd.exec();
}
