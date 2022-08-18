use clap::Parser;
use cli::CargoArgs;

mod cli;
mod new;

fn main() {
    let args = CargoArgs::parse();
    args.exec();
}
