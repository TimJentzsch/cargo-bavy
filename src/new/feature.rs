pub trait Feature: Sized + ToString {
    /// A [`Vec`] containing all available features.
    fn all() -> Vec<Self>;

    /// Determine if a feature should be enabled by default.
    fn enabled_by_default(&self) -> bool;
}
