//! TUI application to display and interactive with the CRS backend

#![warn(clippy::pedantic, clippy::restriction, clippy::nursery)]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "import them all")]
#![allow(
    clippy::implicit_return,
    clippy::question_mark_used,
    clippy::missing_inline_in_public_items,
    clippy::single_call_fn,
    clippy::missing_trait_methods,
    reason = "bad lint"
)]
#![allow(clippy::mod_module_files, reason = "chosen style")]
#![allow(
    clippy::pub_use,
    clippy::wildcard_enum_match_arm,
    clippy::pattern_type_mismatch,
    reason = "dev experience"
)]
#![allow(
    clippy::module_name_repetitions,
    reason = "use objects without module name"
)]

mod app;
mod credentials;
mod tui;
mod ui;

use color_eyre::Result;
use dotenv::dotenv;

use crate::credentials::Credentials;
use crate::tui::Tui;

#[tokio::main]
#[expect(
    clippy::unwrap_in_result,
    reason = "wait to process error to restore the terminal"
)]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenv()?;
    let credentials = Credentials::from_env();

    let mut tui = Tui::new(credentials).await?;
    let res = tui.run().await;
    tui.delete();
    res
}
