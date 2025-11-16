//! Backend for the CRS app.
//!
//! Handles connections to the server and updates the client data on incoming
//! messages.

#![deny(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "import them all")]
#![allow(clippy::multiple_crate_versions, reason = "needed by deps")]
#![allow(clippy::separated_literal_suffix, reason = "chosen style")]
#![allow(
    clippy::implicit_return,
    clippy::question_mark_used,
    clippy::missing_inline_in_public_items,
    clippy::pub_use,
    clippy::module_name_repetitions,
    clippy::single_call_fn,
    reason = "bad lints"
)]

pub mod message;
pub mod room;
pub mod user;
