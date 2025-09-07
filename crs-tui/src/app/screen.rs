use color_eyre::Result;
use ratatui::Frame;
use ratatui::crossterm::event::Event;

use crate::app::chat::ChatPage;
use crate::app::login::LoginPage;
use crate::credentials::Credentials;
use crate::ui::component::Component;

impl Screen {
    fn new_with_login_err(err: String) -> Self {
        Self::Login(LoginPage::new_with_login_err(err))
    }
}

impl Screen {
    fn new(value: Credentials<String>) -> Self {
        Self::Login(LoginPage::from(value))
    }
}

impl Component for Screen {
    type ResponseData = ();
    type UpdateState = ScreenUpdate;

    fn draw(&self, frame: &mut Frame) {
        match self {
            Self::Login(login_page) => login_page.draw(frame),
            Self::Chat(chat_page) => chat_page.draw(frame),
        }
    }

    async fn on_event(
        &mut self,
        event: Event,
    ) -> Result<Option<Self::UpdateState>> {
        match self {
            Self::Login(login_page) => {
                if let Some(login_credentials) =
                    login_page.on_event(event).await?
                {
                    return Ok(Some(ScreenUpdate::AttemptLogIn(
                        login_credentials,
                    )));
                }
            }
            Self::Chat(chat_page) => chat_page.on_event(event).await?.unwrap(),
        }
        Ok(None)
    }

    fn update(&mut self, response_data: Self::ResponseData) {
        match response_data {
            Ok(()) =>
                if matches!(self, Self::Login(_)) {
                    *self = Self::Chat(ChatPage);
                },
            Err(err) =>
                if let Self::Login(login) = self {
                    login.update(err.to_string());
                },
        }
    }
}

/// Informs that the program should exit
pub enum ScreenUpdate {
    /// Attempt to log into the server with these credentials
    AttemptLogIn(Credentials<String>),
}
