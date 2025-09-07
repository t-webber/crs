use backend::matrix_sdk;
use color_eyre::Result;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};

use crate::ui::LoginCredentials;
use crate::ui::component::Component;
use crate::ui::login::LoginPage;

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
    type ResponseData = matrix_sdk::Result<()>;
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

    fn update(&mut self, response_data: Self::ResponseData) {
        let Err(err) = response_data else { todo!() };
        match self {
            Self::Login(login_page) => login_page.update(err.to_string()),
        }
    }
}

/// Informs that the program should exit
pub enum ScreenUpdate {
    /// Attempt to log into the server with these credentials
    AttemptLogIn(LoginCredentials),
    /// The user wan't to exit the app
    ShouldExit,
}
