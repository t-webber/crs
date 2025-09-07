//! Backend for the CRS app.
//!
//! Handles connections to the server and updates the client data on incomming
//! messages.

#![warn(clippy::pedantic, clippy::restriction, clippy::nursery)]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "import them all")]
#![allow(clippy::separated_literal_suffix, reason = "chosen style")]
#![allow(
    clippy::implicit_return,
    clippy::question_mark_used,
    clippy::missing_inline_in_public_items,
    clippy::pub_use,
    reason = "bad lints"
)]

pub mod room;
pub mod user;
