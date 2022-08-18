use super::{
    bevy_features::BevyFeature, compile_features::CompileFeature, project_features::ProjectFeature,
};

type ChangeFn = dyn Fn(&Context);

pub struct Context {
    pub folder_name: String,

    pub bevy_features: Vec<BevyFeature>,
    pub compile_features: Vec<CompileFeature>,
    pub project_features: Vec<ProjectFeature>,

    pub create_files: Vec<CreateFile>,
    pub add_dependencies: Vec<AddDependency>,
    pub extra_changes: Vec<Box<ChangeFn>>,
}

impl Context {
    pub fn new(folder_name: &str) -> Self {
        Self {
            folder_name: folder_name.to_string(),
            bevy_features: Vec::new(),
            compile_features: Vec::new(),
            project_features: Vec::new(),
            create_files: Vec::new(),
            add_dependencies: Vec::new(),
            extra_changes: Vec::new(),
        }
    }
}

pub struct CreateFile {
    pub path: String,
    pub content: String,
}

impl CreateFile {
    pub fn new<P, C>(path: P, content: C) -> Self
    where
        P: Into<String>,
        C: Into<String>,
    {
        Self {
            path: path.into(),
            content: content.into(),
        }
    }
}

pub struct AddDependency {
    pub name: String,
    pub features: Vec<String>,
}
