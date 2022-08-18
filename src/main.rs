use clap::{Parser, Subcommand};

mod new;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(bin_name = "cargo bevy")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    New { folder_name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::New { folder_name } => new::new(folder_name),
    }
}
