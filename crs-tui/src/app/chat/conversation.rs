//! Main chat page with the messages and the inputs to send messages

extern crate alloc;

use alloc::sync::Arc;
use core::convert::Infallible;

use backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{List, ListItem};

use crate::ui::component::Component;

/// Conversation panel, with the all the messages and the input to send messages
pub struct Conversation {
    /// Room currently being displayed
    room: Arc<DisplayRoom>,
}

impl Conversation {
    /// Open a new conversation for the given room
    pub const fn new(room: Arc<DisplayRoom>) -> Self {
        Self { room }
    }
}

impl Component for Conversation {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        match self.room.as_messages() {
            Ok(messages) => {
                let list = messages
                    .iter()
                    .map(|message| ListItem::new(message.as_body()));
                frame.render_widget(List::new(list), area);
            }
            Err(err) => {
                frame.render_widget(
                    Text::from(err.to_string())
                        .style(Style::default().fg(Color::Red)),
                    area,
                );
            }
        }
    }
}
