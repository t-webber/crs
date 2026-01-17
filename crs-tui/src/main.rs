//! TUI application to display and interactive with the CRS backend

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
#![allow(
    clippy::implicit_return,
    clippy::question_mark_used,
    clippy::missing_inline_in_public_items,
    clippy::single_call_fn,
    clippy::missing_trait_methods,
    clippy::else_if_without_else,
    reason = "bad lint"
)]
#![allow(
    clippy::mod_module_files,
    clippy::separated_literal_suffix,
    reason = "chosen style"
)]
#![allow(
    dead_code,
    clippy::pub_use,
    clippy::wildcard_enum_match_arm,
    clippy::pattern_type_mismatch,
    clippy::indexing_slicing,
    clippy::missing_asserts_for_indexing,
    reason = "dev experience"
)]
#![allow(
    clippy::module_name_repetitions,
    reason = "we use objects without module name"
)]
#![allow(clippy::multiple_crate_versions, reason = "needed by deps")]

mod app;
mod credentials;
mod tui;
mod ui;
mod utils;

use dotenv::dotenv;

use crate::credentials::Credentials;
use crate::tui::Tui;

#[tokio::main]
#[expect(clippy::unwrap_in_result, reason = "needed by tokio")]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    dotenv()?;
    let credentials = Credentials::from_env();
    let mut tui = Tui::new(credentials).await?;
    let res = tui.run().await;
    tui.delete();

    res.map_err(Into::into)
}
