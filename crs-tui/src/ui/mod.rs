//! Handles the display to the screen and event listening

mod components;
mod login;

use color_eyre::Result;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};

pub use crate::ui::login::LoginCredentials;
use crate::ui::login::LoginPage;

/// Trait to define components present in the app
///
/// An infinite loop is executed to re-redner the app. The UI is rendered, then
/// blocked until an [`Event`] occurs. After when this event occurs, the
/// component is updated then the screen is redrawn.
///
/// The struct deriving the component should contain the data needed for the
/// [`Self::render`] method, and nothing else.
///
/// If events happen that depend on this component, the [`Self::on_event`] is
/// called. It returns an [`Self::UpdateState`] that is used to send information
/// to the parent component if need be.
pub trait Component {
    /// Data returned to the parent component on update
    ///
    /// It is a good practice to mark the types used here as `must_use`, to
    /// prevent the parent from discarding it.
    type UpdateState;

    /// Renders the component on the given frame
    fn draw(&self, frame: &mut Frame);

    /// Listens for events that are custom to this UI component
    ///
    /// # Returns
    ///
    /// - `Ok(Some(state))` if there is something to be done by the parent
    ///   component
    /// - `Ok(None)` if nothing else needs to be done
    /// - `Err(err)` if something unexpected occured
    #[expect(unused_variables, reason = "trait def")]
    fn on_event(&mut self, event: Event) -> Result<Option<Self::UpdateState>> {
        Ok(None)
    }
}

/// Screen currently displayed on the user interface
pub enum Screen {
    /// Page to prompt for matrix credentials
    Login(LoginPage),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Login(LoginPage::default())
    }
}

impl Component for Screen {
    type UpdateState = ScreenUpdate;

    fn draw(&self, frame: &mut Frame) {
        match self {
            Self::Login(login_page) => login_page.draw(frame),
        }
    }

    fn on_event(&mut self, event: Event) -> Result<Option<Self::UpdateState>> {
        if let Event::Key(key_event) = event
            && key_event.code == KeyCode::Esc
        {
            return Ok(Some(ScreenUpdate::ShouldExit));
        }
        match self {
            Self::Login(login_page) => {
                if let Some(login_credentials) = login_page.on_event(event)? {
                    return Ok(Some(ScreenUpdate::AttemptLogIn(
                        login_credentials,
                    )));
                }
            }
        }
        Ok(None)
    }
}

/// Informs that the program should exit
pub enum ScreenUpdate {
    /// Attempt to log into the server with these credentials
    AttemptLogIn(LoginCredentials),
    /// The user wan't to exit the app
    ShouldExit,
}
