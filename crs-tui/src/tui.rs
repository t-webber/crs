//! Holds the data and manages the terminal for the TUI

use core::time::Duration;
use std::io::{self, Stdout};

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{Event, KeyCode, poll, read};

use crate::app::App;
use crate::credentials::Credentials;
use crate::ui::component::Component as _;

/// Holds the data and the state of the TUI
pub struct Tui {
    /// Current screen displayed on the TUI
    app:      App,
    /// Terminal on which the TUI is drawn
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

#[expect(clippy::arbitrary_source_item_ordering, reason = "run order")]
impl Tui {
    /// Draws the UI in the terminal
    fn draw(&mut self) -> Result<(), io::Error> {
        self.terminal.draw(|frame| self.app.draw(frame, frame.area()))?;
        Ok(())
    }

    /// Creates a new instance of [`Self`]
    ///
    /// This functions enters the terminal in raw mode. Please call
    /// [`Self::delete`] before exiting the program. Make sure errors don't
    /// stop the program before [`Self::delete`] is called.
    ///
    /// # Errors
    ///
    /// Returns an error when the login was not successful.
    pub async fn new(
        credentials: Credentials<Option<String>>,
    ) -> color_eyre::Result<Self> {
        let app = if credentials.is_full() {
            let user = credentials.fill_with_empty().login().await?;
            App::new_with_user(user)
        } else {
            App::from(credentials.fill_with_empty())
        };

        Ok(Self { terminal: ratatui::init(), app })
    }

    /// Handles user events
    ///
    /// # Returns
    ///
    /// Returns `true` if the app should exit, `false` otherwise.
    async fn on_event(&mut self, event: Event) -> Result<bool, io::Error> {
        if let Event::Key(key_event) = event
            && key_event.code == KeyCode::Esc
        {
            return Ok(true);
        }
        if let Some(credentials) = self.app.on_event(event).await {
            self.draw()?;
            match credentials.login().await {
                Ok(user) => self.app = App::new_with_user(user),
                Err(err) => self.app = App::new_with_login_err(err.to_string()),
            }
        }
        Ok(false)
    }

    /// Runs the TUI
    ///
    /// The TUI is drawn, then waits for events: key pressed, mouse clicked,
    /// window resized, etc. and handles that event.
    ///
    /// Once the event is handled, the UI components are updated and redrawn.
    ///
    /// If no event occurs, the app is refr5sh every 200 ms.
    pub async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            self.draw()?;
            if poll(Duration::from_millis(200))? {
                let event = read()?;
                if self.on_event(event).await? {
                    break Ok(());
                }
            }
        }
    }

    /// Deletes the app and cleans up
    ///
    /// This must be run even if the app crashes to restore the terminal to
    /// normal usage. If forgotten, the user can restore the terminal with the
    /// `stty sane` (on Linux).
    #[expect(clippy::unused_self, reason = "the goal is to destroy the object")]
    pub fn delete(self) {
        ratatui::restore();
    }
}
