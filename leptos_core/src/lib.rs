#![deny(missing_docs)]

//! This crate contains several utility pieces that depend on multiple crates.
//! They are all re-exported in the main `leptos` crate.

mod for_component;
mod map;
mod suspense;
mod transition;

pub use for_component::*;
pub use map::*;
pub use suspense::*;
pub use transition::*;
pub use typed_builder;

/// Describes the properties of a component. This is typically generated by the `Prop` derive macro
/// as part of the `#[component]` macro.
pub trait Prop {
    /// Builder type, automatically generated.
    type Builder;

    /// The builder should be automatically generated using the `Prop` derive macro.
    fn builder() -> Self::Builder;
}
