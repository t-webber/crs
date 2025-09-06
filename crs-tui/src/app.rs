//! App struct to hold the data and state of the TUI

use std::io::Stdout;

use backend::user::User;
use color_eyre::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::read;

use crate::ui::{Component as _, LoginCredentials, Screen, ScreenUpdate};

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
            screen:   Screen::default(),
        })
    }

    /// Runs the TUI
    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.terminal.draw(|frame| self.screen.draw(frame))?;
            let Some(update) = self.screen.on_event(read()?)? else { continue };
            match update {
                ScreenUpdate::ShouldExit => break Ok(()),
                ScreenUpdate::AttemptLogIn(LoginCredentials {
                    username,
                    password,
                }) => self.user.login(username, &password).await?,
            }
        }
    }

    /// Deletes the app and clean up
    #[expect(clippy::unused_self, reason = "the goal is to destroy the object")]
    pub fn delete(self) {
        ratatui::restore();
    }
}
