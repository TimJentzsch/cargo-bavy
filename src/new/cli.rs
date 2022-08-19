use clap::Args;

use crate::cli::Command;

use super::new;

#[derive(Debug, Args)]
pub struct NewCommand {
    folder_name: String,
}

impl Command for NewCommand {
    fn exec(&self) {
        new(&self.folder_name);
    }
}
