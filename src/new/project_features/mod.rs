pub mod license;

use self::license::add_licenses;

use super::{context::Context, feature::Feature, utils::select_features};
use license::get_copyright_info;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProjectFeature {
    MitApacheLicenses,
}

impl Feature for ProjectFeature {
    /// A [`Vec`] containing all available project features.
    fn all() -> Vec<Self> {
        vec![ProjectFeature::MitApacheLicenses]
    }

    /// Determines if a feature should be enabled by default.
    fn enabled_by_default(&self) -> bool {
        true
    }
}

impl ToString for ProjectFeature {
    fn to_string(&self) -> String {
        match self {
            ProjectFeature::MitApacheLicenses => {
                format!(
                    "MIT and Apache-2.0 licenses (Copyright (c) {})",
                    get_copyright_info()
                )
            }
        }
    }
}

pub fn select_project_features() -> Vec<ProjectFeature> {
    select_features("Which compile features do you want?")
}

pub fn register_project_features(context: &mut Context) {
    add_licenses(context);
}
