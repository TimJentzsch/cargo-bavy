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
            CargoCommand::Bavy(args) => args.exec(),
        }
    }
}

#[derive(Debug, Subcommand)]
enum CargoCommand {
    Bavy(BavyArgs),
}

#[derive(Debug, Args)]
#[clap(version, about, long_about = None)]
struct BavyArgs {
    #[clap(subcommand)]
    command: BavyCommand,
}

impl BavyArgs {
    pub fn exec(&self) {
        match &self.command {
            BavyCommand::New(args) => {
                args.exec();
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum BavyCommand {
    New(NewArgs),
}
