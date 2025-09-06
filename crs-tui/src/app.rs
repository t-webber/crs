//! App struct to hold the data and state of the TUI


use std::io::Stdout;

use backend::user::User;
use color_eyre::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

/// Screen currently displayed on the user interface
struct Screen;

/// Holds the data and the state of the TUI
pub struct App {
    /// Current screen displayed on the TUI
    screen:   Screen,
    /// Terminal on which the TUI is drawn
    terminal: Terminal<CrosstermBackend<Stdout>>,
    /// Backend user to interact with the homeserver
    user:     User,
}

#[expect(clippy::arbitrary_source_item_ordering, reason = "run order")]
impl App {
    /// Creates a new instance of [`Self`]
    pub async fn new(homeserver_url: &str) -> Result<Self> {
        Ok(Self {
            user:     User::new(homeserver_url).await?,
            terminal: ratatui::init(),
            screen:   Screen,
        })
    }

    /// Runs the TUI
    pub const fn run(&mut self) -> Result<()> {
        Ok(())
    }

    /// Deletes the app and clean up
    pub fn delete(self) {
        ratatui::restore();
    }
}
