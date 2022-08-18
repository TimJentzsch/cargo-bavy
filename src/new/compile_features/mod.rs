mod dependencies;
mod fast_linker;
mod nightly;
mod wasm;

use self::{
    dependencies::optimize_dependencies, fast_linker::add_fast_linker,
    nightly::add_nightly_toolchain, wasm::add_wasm,
};

use super::{context::Context, feature::Feature, utils::select_features};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompileFeature {
    NightlyToolchain,
    FastLinker,
    OptimizeDependencies,
    WasmTarget,
}

impl Feature for CompileFeature {
    /// A [`Vec`] containing all available compile features.
    fn all() -> Vec<Self> {
        vec![
            CompileFeature::NightlyToolchain,
            CompileFeature::FastLinker,
            CompileFeature::OptimizeDependencies,
            CompileFeature::WasmTarget,
        ]
    }

    /// Determines if a feature should be enabled by default.
    fn enabled_by_default(&self) -> bool {
        true
    }
}

impl ToString for CompileFeature {
    fn to_string(&self) -> String {
        match self {
            CompileFeature::NightlyToolchain => "Nightly toolchain".to_string(),
            CompileFeature::FastLinker => "Fast linker (LLD/ZLD)".to_string(),
            CompileFeature::OptimizeDependencies => {
                "Optimize dependencies in debug mode".to_string()
            }
            CompileFeature::WasmTarget => "Target WASM".to_string(),
        }
    }
}

pub fn select_compile_features() -> Vec<CompileFeature> {
    select_features("[2/3] Which compile features do you want?")
}

pub fn register_compile_features(context: &mut Context) {
    if context
        .compile_features
        .contains(&CompileFeature::NightlyToolchain)
    {
        add_nightly_toolchain(context);
    }

    if context
        .compile_features
        .contains(&CompileFeature::FastLinker)
    {
        add_fast_linker(context);
    }

    if context
        .compile_features
        .contains(&CompileFeature::WasmTarget)
    {
        add_wasm(context);
    }

    if context
        .compile_features
        .contains(&CompileFeature::OptimizeDependencies)
    {
        optimize_dependencies(context);
    }
}
