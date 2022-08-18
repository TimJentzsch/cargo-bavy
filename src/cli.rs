use clap::{Args, Parser, Subcommand};

use crate::new::cli::NewArgs;

#[derive(Debug, Parser)]
#[clap(bin_name = "cargo")]
pub struct CargoArgs {
    #[clap(subcommand)]
    command: CargoCommand,
}

impl CargoArgs {
    pub fn exec(&self) {
        match &self.command {
            CargoCommand::Bevy(args) => args.exec(),
        }
    }
}

#[derive(Debug, Subcommand)]
enum CargoCommand {
    Bevy(BevyArgs),
}

#[derive(Debug, Args)]
#[clap(version, about, long_about = None)]
struct BevyArgs {
    #[clap(subcommand)]
    command: BevyCommand,
}

impl BevyArgs {
    pub fn exec(&self) {
        match &self.command {
            BevyCommand::New(args) => {
                args.exec();
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum BevyCommand {
    New(NewArgs),
}
