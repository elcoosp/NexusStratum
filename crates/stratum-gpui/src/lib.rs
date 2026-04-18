//! # stratum-gpui
//!
//! GPUI adapter for NexusStratum headless primitives.
//!
//! Currently provides simple Checkbox and Switch components.

pub mod bindable;
pub mod components;

pub use components::checkbox::Checkbox;
pub use components::switch::Switch;
