mod feature;
mod license;
mod utils;

use std::{env, process::Command};

use dialoguer::MultiSelect;
use license::add_licenses;

use crate::feature::Feature;

fn main() {
    let folder_name = env::args()
        .nth(1)
        .expect("Please specify the name of the project.");

    let features = select_features();

    create_bevy_app(&folder_name, features);
}

fn select_features() -> Vec<Feature> {
    println!("Which features do you want?");
    println!("([Space] to select, [Enter] to confirm.)");

    let features = Feature::all();
    let selection = MultiSelect::new()
        .items(&features)
        .defaults(
            features
                .iter()
                .map(|feature| feature.enabled_by_default())
                .collect::<Vec<bool>>()
                .as_slice(),
        )
        .interact()
        .unwrap();

    features
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| selection.contains(idx))
        .map(|(_idx, feature)| feature)
        .collect()
}

fn create_bevy_app(folder_name: &str, features: Vec<Feature>) {
    create_cargo_app(folder_name);

    if features.contains(&Feature::MitApacheLicenses) {
        add_licenses(folder_name)
    }
}

fn create_cargo_app(folder_name: &str) {
    let output = Command::new("cargo")
        .arg("new")
        .arg(folder_name)
        .status()
        .expect("Failed to create the new project.")
        .success();

    if !output {
        panic!("Failed to create the new project.");
    }
}
