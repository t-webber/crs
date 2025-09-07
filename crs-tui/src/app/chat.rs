//! Main page displayed with the chats

use std::rc::Rc;

use backend::user::User;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::Text;

use crate::ui::component::Component;

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct ChatPage {
    /// User to interact with matrix server
    user: Rc<User>,
}

impl ChatPage {
    pub fn new(user: Rc<User>) -> Self {
        Self { user }
    }
}

impl Component for ChatPage {
    type ResponseData = ();
    type UpdateState = ();

    #[expect(clippy::indexing_slicing, reason = "len = 2")]
    fn draw(&self, frame: &mut ratatui::Frame) {
        if frame.area().width <= 30 {
            todo!()
        }

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(frame.area());

        frame.render_widget(Text::from("hi"), layout[1]);
    }
}
