//! TUI application to display and interactive with the CRS backend

#![warn(clippy::pedantic, clippy::restriction, clippy::nursery)]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "import them all")]
#![allow(
    clippy::implicit_return,
    clippy::question_mark_used,
    clippy::missing_inline_in_public_items,
    clippy::single_call_fn,
    reason = "bad lint"
)]

mod app;

use crate::app::App;


#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let mut app = App::new("http://localhost:8008").await?;
    let res = app.run();
    app.delete();
    res
}
