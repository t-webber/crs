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
mod ui;

use std::env;

use color_eyre::Result;
use color_eyre::eyre::Context as _;
use dotenv::dotenv;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenv()?;
    let homeserver_url = env::var("HOMESERVER_URL").with_context(|| {
        "Please add the HOMESERVER_URL variable in the .env file"
    })?;
    let mut app = App::new(&homeserver_url).await?;
    let res = app.run().await;
    app.delete();
    res
}
