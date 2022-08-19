use clap::Parser;
use cli::{CargoCommand, Command};

mod cli;
mod new;

fn main() {
    let cmd = CargoCommand::parse();
    cmd.exec();
}
