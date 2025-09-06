//! App struct to hold the data and state of the TUI

/// Holds the data and the state of the TUI
pub struct App;

#[expect(clippy::arbitrary_source_item_ordering, reason = "run order")]
impl App {
    /// Creates a new instance of [`Self`]
    pub fn new() -> Self {
        ratatui::init();
        Self
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
