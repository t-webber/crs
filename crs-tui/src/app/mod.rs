//! This  module contains the app, with both UI state and  matrix user.

use std::rc::Rc;

use backend::user::User;
use ratatui::crossterm::event::Event;

use crate::app::chat::ChatPage;
use crate::app::login::LoginPage;
use crate::credentials::Credentials;
use crate::ui::component::Component;

mod chat;
mod login;

pub struct App {
    screen: Screen,
    user:   Option<Rc<User>>,
}

impl App {
    fn new_with_login_err(err: String) -> Self {
        Self {
            screen: Screen::Login(LoginPage::new_with_login_err(err)),
            user:   None,
        }
    }
}

impl Component for App {
    type ResponseData = ();
    type UpdateState = ();

    fn draw(&self, frame: &mut ratatui::Frame) {
        match &self.screen {
            Screen::Login(login) => login.draw(frame),
            Screen::Chat(chat) => chat.draw(frame),
        }
    }

    async fn on_event(
        &mut self,
        event: Event,
    ) -> color_eyre::Result<Option<Self::UpdateState>> {
        match &mut self.screen {
            Screen::Login(login_page) =>
                if let Some(credentials) = login_page.on_event(event).await? {
                    match credentials.login().await {
                        Ok(user) => *self = Self::from(user),
                        Err(err) =>
                            *self = Self::new_with_login_err(err.to_string()),
                    }
                },
            Screen::Chat(chat_page) => {
                chat_page.on_event(event).await?;
            }
        }
        Ok(None)
    }
}

impl From<Credentials<String>> for App {
    fn from(value: Credentials<String>) -> Self {
        Self { screen: Screen::Login(LoginPage::new(value)), user: None }
    }
}

impl From<User> for App {
    fn from(value: User) -> Self {
        let user = Rc::new(value);
        Self {
            screen: Screen::Chat(ChatPage::new(user.clone())),
            user:   Some(user),
        }
    }
}

/// Screen currently displayed on the user interface
#[expect(clippy::arbitrary_source_item_ordering, reason = "chronological")]
pub enum Screen {
    /// Page to prompt for matrix credentials
    Login(LoginPage),
    /// Main page to chat
    Chat(ChatPage),
}
