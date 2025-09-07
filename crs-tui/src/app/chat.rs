//! Main page displayed with the chats

extern crate alloc;
use alloc::sync::Arc;

use backend::user::User;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Text;

use crate::ui::component::Component;

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct ChatPage {
    /// User to interact with matrix server
    user: Arc<User>,
}

impl ChatPage {
    /// Create a new chat page with the given logged in user
    pub const fn new(user: Arc<User>) -> Self {
        Self { user }
    }
}

impl Component for ChatPage {
    type ResponseData = ();
    type UpdateState = ();

    #[expect(clippy::indexing_slicing, reason = "len = 2")]
    fn draw(&self, frame: &mut Frame, area: Rect) {
        if frame.area().width <= 30 {
            todo!()
        }

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(area);

        frame.render_widget(Text::from("hi"), layout[1]);
    }
}
