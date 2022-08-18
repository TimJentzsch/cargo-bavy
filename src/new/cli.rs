use clap::Args;

use super::new;

#[derive(Debug, Args)]
pub struct NewArgs {
    folder_name: String,
}

impl NewArgs {
    pub fn exec(&self) {
        new(&self.folder_name);
    }
}
