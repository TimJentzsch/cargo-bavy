use super::license::get_copyright_info;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Feature {
    FastCompileTimes,
    AssetHotReloading,
    WasmSupport,
    MitApacheLicenses,
}

impl Feature {
    /// A [`Vec`] containing all available features.
    pub fn all() -> Vec<Self> {
        vec![
            Feature::FastCompileTimes,
            Feature::AssetHotReloading,
            Feature::WasmSupport,
            Feature::MitApacheLicenses,
        ]
    }

    /// Determines if a feature should be enabled by default.
    pub fn enabled_by_default(&self) -> bool {
        true
    }
}

impl ToString for Feature {
    fn to_string(&self) -> String {
        match self {
            Feature::FastCompileTimes => "Fast compile times".to_string(),
            Feature::AssetHotReloading => "Hot reloading for assets".to_string(),
            Feature::WasmSupport => "WASM support".to_string(),
            Feature::MitApacheLicenses => {
                format!(
                    "MIT and Apache-2.0 licenses (Copyright (c) {})",
                    get_copyright_info()
                )
            }
        }
    }
}
