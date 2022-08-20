use clap::{Args, Parser, Subcommand};

use crate::{build::cli::BuildCommand, new::cli::NewCommand, run::cli::RunCommand};

pub trait Command {
    /// Execute the command
    fn exec(&self);
}

#[derive(Debug, Parser)]
#[clap(bin_name = "cargo")]
pub struct CargoCommand {
    #[clap(subcommand)]
    command: CargoSubcommand,
}

impl Command for CargoCommand {
    fn exec(&self) {
        self.command.exec();
    }
}

#[derive(Debug, Subcommand)]
enum CargoSubcommand {
    Bavy(BavyCommand),
}

impl Command for CargoSubcommand {
    fn exec(&self) {
        match self {
            CargoSubcommand::Bavy(cmd) => cmd.exec(),
        }
    }
}

#[derive(Debug, Args)]
#[clap(version, about, long_about = None)]
struct BavyCommand {
    #[clap(subcommand)]
    command: BavySubcommand,
}

impl Command for BavyCommand {
    fn exec(&self) {
        self.command.exec();
    }
}

#[derive(Debug, Subcommand)]
enum BavySubcommand {
    /// Create a new Bevy app.
    New(NewCommand),
    /// Run your Bevy app with optimized compile times.
    Run(RunCommand),
    /// Build your Bevy app with optimized compile times.
    Build(BuildCommand),
}

impl Command for BavySubcommand {
    fn exec(&self) {
        match self {
            BavySubcommand::New(cmd) => cmd.exec(),
            BavySubcommand::Run(cmd) => cmd.exec(),
            BavySubcommand::Build(cmd) => cmd.exec(),
        }
    }
}
