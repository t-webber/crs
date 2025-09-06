//! App struct to hold the data and state of the TUI

use backend::user::User;
use color_eyre::Result;

/// Holds the data and the state of the TUI
pub struct App {
    /// Backend user to interact with the homeserver
    user: User,
}

#[expect(clippy::arbitrary_source_item_ordering, reason = "run order")]
impl App {
    /// Creates a new instance of [`Self`]
    pub async fn new(homeserver_url: &str) -> Result<Self> {
        ratatui::init();
        Ok(Self { user: User::new(homeserver_url).await? })
    }

    /// Runs the TUI
    pub fn run(&mut self) -> color_eyre::Result<()> {
        Ok(())
    }

    /// Deletes the app and clean up
    pub fn delete(self) {
        ratatui::restore();
    }
}
