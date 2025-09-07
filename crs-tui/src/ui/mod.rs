//! This part of the crate handles everything related to the UI.
//!
//! The only states present here are those necessary to display the components
//! of the TUI, but the logic will be found in the app module.

pub mod component;
mod login;
pub mod screen;
mod widgets;

pub use login::LoginCredentials;
