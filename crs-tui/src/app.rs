//! App struct to hold the data and state of the TU

use std::io::Stdout;

use backend::user::User;
use color_eyre::Result;
use color_eyre::eyre::Context as _;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::read;

use crate::ui::LoginCredentials;
use crate::ui::component::Component as _;
use crate::ui::screen::{Screen, ScreenUpdate};

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
    ///
    /// # Errors
    ///
    /// Returns an error when
    /// [`ClientBuild::build`](backend::matrix_sdk::ClientBuilder::build) does.
    pub async fn new(homeserver_url: &str) -> Result<Self> {
        Ok(Self {
            user:     User::new(homeserver_url).await?,
            terminal: ratatui::init(),
            screen:   Screen::default(),
        })
    }

    /// Runs the TUI
    ///
    /// The TUI is drawn, then waits for events: key pressed, mouse clicked,
    /// window resized, etc. and handles that event.
    ///
    /// Once the event is handled, the UI components are updated and the ²;
    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.terminal.draw(|frame| self.screen.draw(frame))?;
            let Some(update) = self.screen.on_event(read()?)? else { continue };
            match update {
                ScreenUpdate::ShouldExit => break Ok(()),
                ScreenUpdate::AttemptLogIn(LoginCredentials {
                    username,
                    password,
                }) => self
                    .screen
                    .update(self.user.login(username, &password).await),
            }
        }
    }

    /// Deletes the app and clean up
    #[expect(clippy::unused_self, reason = "the goal is to destroy the object")]
    pub fn delete(self) {
        ratatui::restore();
    }
}
