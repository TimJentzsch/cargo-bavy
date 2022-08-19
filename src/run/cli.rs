use clap::Args;

use crate::cli::Command;

use super::run;

#[derive(Debug, Args)]
pub struct RunCommand {}

impl Command for RunCommand {
    fn exec(&self) {
        run();
    }
}
