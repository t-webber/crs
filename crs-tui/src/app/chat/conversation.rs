//! Main chat page with the messages and the inputs to send messages

extern crate alloc;

use alloc::sync::Arc;
use core::convert::Infallible;

use backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::layout::Rect;
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

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let list = List::new(
            self.room
                .as_messages()
                .unwrap()
                .iter()
                .map(|message| ListItem::new(message.as_body())),
        );
        frame.render_widget(list, area);
    }
}
