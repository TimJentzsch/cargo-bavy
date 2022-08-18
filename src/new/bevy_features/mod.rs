use super::{feature::Feature, utils::select_features};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BevyFeature {
    AssetHotReloading,
}

impl Feature for BevyFeature {
    /// A [`Vec`] containing all available Bevy features.
    fn all() -> Vec<Self> {
        vec![BevyFeature::AssetHotReloading]
    }

    /// Determines if a feature should be enabled by default.
    fn enabled_by_default(&self) -> bool {
        true
    }
}

impl ToString for BevyFeature {
    fn to_string(&self) -> String {
        match self {
            BevyFeature::AssetHotReloading => "Hot reloading for assets".to_string(),
        }
    }
}

pub fn select_bevy_features() -> Vec<BevyFeature> {
    select_features("Which compile features do you want?")
}
