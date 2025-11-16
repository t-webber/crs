//! This  module contains the app, with both UI state and  matrix user.

mod chat;
mod login;

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;

use crs_backend::user::User;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::chat::ChatPage;
use crate::app::login::LoginPage;
use crate::credentials::Credentials;
use crate::ui::component::Component;
use crate::ui::widgets::{fully_centred_content, saturating_cast};

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
    pub fn new_with_login_err(err: String) -> Self {
        Self {
            screen: Screen::Login(LoginPage::new_with_login_err(err)),
            user:   None,
        }
    }

    /// Creates a new page with a logged in user
    pub fn new_with_user(user: User) -> Self {
        let sharable_user = Arc::new(user);
        let _handle = sharable_user.enable_sync();
        let page = ChatPage::new(Arc::clone(&sharable_user));
        Self { screen: Screen::Chat(page), user: Some(sharable_user) }
    }
}

impl Component for App {
    type ResponseData = Infallible;
    type UpdateState = Credentials<String>;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        if area.width <= 20 || area.height <= 10 {
            let text = "Terminal too small.";
            let rect = fully_centred_content(
                saturating_cast(text.len()),
                area.width,
                area,
            );
            frame.render_widget(
                Paragraph::new(text).centered().wrap(Wrap { trim: true }),
                rect,
            );
            return;
        }

        match &self.screen {
            Screen::Login(login) => login.draw(frame, area),
            Screen::Chat(chat) => chat.draw(frame, area),
        }
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        match &mut self.screen {
            Screen::Login(login_page) => login_page.on_event(event).await,
            Screen::Chat(chat_page) => {
                chat_page.on_event(event).await?;
                None
            }
        }
    }
}

impl From<Credentials<String>> for App {
    fn from(value: Credentials<String>) -> Self {
        Self { screen: Screen::Login(LoginPage::new(value)), user: None }
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
