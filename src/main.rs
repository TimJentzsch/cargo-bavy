use std::{
    env,
    io::{self, Write},
    process::Command,
};

fn main() {
    let folder_name = env::args()
        .nth(1)
        .expect("Please specify the name of the project.");

    let output = Command::new("cargo")
        .arg("new")
        .arg(folder_name)
        .output()
        .expect("Failed to create the new project.");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
