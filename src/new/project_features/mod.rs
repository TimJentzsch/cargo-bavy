mod license;
mod workflows;

use self::{
    license::add_licenses,
    workflows::{add_ci_workflow, add_release_workflow},
};

use super::{context::Context, feature::Feature, utils::select_features};
use license::get_copyright_info;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProjectFeature {
    MitApacheLicenses,
    CiWorkflow,
    ReleaseWorkflow,
}

impl Feature for ProjectFeature {
    /// A [`Vec`] containing all available project features.
    fn all() -> Vec<Self> {
        vec![
            ProjectFeature::MitApacheLicenses,
            ProjectFeature::CiWorkflow,
            ProjectFeature::ReleaseWorkflow,
        ]
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
            ProjectFeature::CiWorkflow => "CI GitHub Actions workflow".to_string(),
            ProjectFeature::ReleaseWorkflow => "Release GitHub Actions workflow".to_string(),
        }
    }
}

pub fn select_project_features() -> Vec<ProjectFeature> {
    select_features("Which compile features do you want?")
}

pub fn register_project_features(context: &mut Context) {
    if context
        .project_features
        .contains(&ProjectFeature::MitApacheLicenses)
    {
        add_licenses(context);
    }

    if context
        .project_features
        .contains(&ProjectFeature::CiWorkflow)
    {
        add_ci_workflow(context);
    }

    if context
        .project_features
        .contains(&ProjectFeature::ReleaseWorkflow)
    {
        add_release_workflow(context);
    }
}
