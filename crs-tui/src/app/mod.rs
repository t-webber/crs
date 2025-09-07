//! This  module contains the app, with both UI state and  matrix user.

mod chat;
mod login;

extern crate alloc;
use alloc::sync::Arc;

use backend::user::User;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;

use crate::app::chat::ChatPage;
use crate::app::login::LoginPage;
use crate::credentials::Credentials;
use crate::ui::component::Component;

/// App containing the user, the UI state
///
/// This app is responsible of knowing which screen to render, and how to switch
/// between them
pub struct App {
    /// Current screen rendered on in the app
    screen: Screen,
    /// Logged in user to communicate with the homeserver
    user:   Option<Arc<User>>,
}

impl App {
    /// Create a new page after an error at login
    ///
    /// This will repon the login page with the error message
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

    fn draw(&self, frame: &mut Frame, area: Rect) {
        match &self.screen {
            Screen::Login(login) => login.draw(frame, area),
            Screen::Chat(chat) => chat.draw(frame, area),
        }
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        match &mut self.screen {
            Screen::Login(login_page) =>
                if let Some(credentials) = login_page.on_event(event).await {
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
        None
    }
}

impl From<Credentials<String>> for App {
    fn from(value: Credentials<String>) -> Self {
        Self { screen: Screen::Login(LoginPage::new(value)), user: None }
    }
}

impl From<User> for App {
    fn from(value: User) -> Self {
        let user = Arc::new(value);
        Self {
            screen: Screen::Chat(ChatPage::new(Arc::clone(&user))),
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
