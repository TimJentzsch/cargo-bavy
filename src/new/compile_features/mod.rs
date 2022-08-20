mod dependencies;
mod fast_linker;
mod nightly;
mod rust_analyzer_bevy_dynamic;
mod wasm;

use dialoguer::console::style;

use self::{
    dependencies::optimize_dependencies, fast_linker::add_fast_linker,
    nightly::add_nightly_toolchain,
    rust_analyzer_bevy_dynamic::enable_bevy_dynamic_for_rust_analyzer, wasm::add_wasm,
};

use super::{context::Context, feature::Feature, utils::select_features};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompileFeature {
    NightlyToolchain,
    FastLinker,
    OptimizeDependencies,
    WasmTarget,
    RustAnalyzerBevyDynamic,
}

impl Feature for CompileFeature {
    /// A [`Vec`] containing all available compile features.
    fn all() -> Vec<Self> {
        vec![
            CompileFeature::NightlyToolchain,
            CompileFeature::FastLinker,
            CompileFeature::OptimizeDependencies,
            CompileFeature::WasmTarget,
            CompileFeature::RustAnalyzerBevyDynamic,
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
            CompileFeature::WasmTarget => {
                "Target WASM with `wasm32-unknown-unknown` and `wasm-bindgen-cli`".to_string()
            }
            CompileFeature::RustAnalyzerBevyDynamic => {
                "Enable `bevy/dynamic` in VS Code for Rust Analyzer (faster compiles)".to_string()
            }
        }
    }
}

pub fn select_compile_features() -> Vec<CompileFeature> {
    select_features(format!(
        "{} Which {} do you want?",
        style("[2/3]").black(),
        style("compile features").cyan(),
    ))
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

    if context
        .compile_features
        .contains(&CompileFeature::RustAnalyzerBevyDynamic)
    {
        enable_bevy_dynamic_for_rust_analyzer(context);
    }
}
