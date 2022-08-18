use std::{env, process::Command};

use dialoguer::MultiSelect;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Feature {
    FastCompileTimes,
    AssetHotReloading,
    WasmSupport,
    MitApacheLicenses,
}

impl Feature {
    /// A [`Vec`] containing all available features.
    fn all() -> Vec<Self> {
        vec![
            Feature::FastCompileTimes,
            Feature::AssetHotReloading,
            Feature::WasmSupport,
            Feature::MitApacheLicenses,
        ]
    }

    /// Determines if a feature should be enabled by default.
    fn enabled_by_default(&self) -> bool {
        true
    }
}

impl ToString for Feature {
    fn to_string(&self) -> String {
        match self {
            Feature::FastCompileTimes => "Fast compile times".to_string(),
            Feature::AssetHotReloading => "Hot reloading for assets".to_string(),
            Feature::WasmSupport => "WASM support".to_string(),
            Feature::MitApacheLicenses => "MIT and Apache-2.0 licenses".to_string(),
        }
    }
}

fn main() {
    let folder_name = env::args()
        .nth(1)
        .expect("Please specify the name of the project.");

    let features = select_features();
    println!("{features:?}");
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

fn create_project(folder_name: String) {
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
