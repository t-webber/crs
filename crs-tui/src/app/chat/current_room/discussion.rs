//! Main chat page with the messages and the inputs to send messages

extern crate alloc;

use alloc::sync::Arc;
use core::convert::Infallible;
use std::sync::Mutex;

use crs_backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{List, ListItem};

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::utils::safe_unlock;

/// Discussion panel, with the all the messages and the input to send messages
pub struct Discussion {
    /// Message prompt to write the messages
    message_prompt: Input<'static>,
    /// Room currently being displayed
    room:           Arc<Mutex<DisplayRoom>>,
}

impl Discussion {
    /// Returns the underlying [`DisplayRoom`]
    pub fn into_room(self) -> Arc<Mutex<DisplayRoom>> {
        self.room
    }

    /// Open a new conversation for the given room
    pub const fn new(room: Arc<Mutex<DisplayRoom>>) -> Self {
        Self { room, message_prompt: Input::new().with_active() }
    }

    /// Checks if the current room is the same that the provided one, by
    /// checking their ids.
    pub fn room_is(&self, other: &DisplayRoom) -> bool {
        safe_unlock(&self.room).id() == other.id()
    }
}

impl Component for Discussion {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    #[expect(
        clippy::unwrap_used,
        reason = "channel can't become erroneous on update"
    )]
    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let layout = Layout::new(Direction::Vertical, [
            Constraint::Fill(1),
            Constraint::Length(Input::HEIGHT_WITHOUT_LABEL),
        ])
        .split(area);

        let room = safe_unlock(&self.room);
        let messages = room.as_messages().unwrap();

        let list = messages.iter().map(|message| {
            ListItem::new(format!(
                "{}: {}",
                message.as_sender(),
                message.as_body()
            ))
        });

        frame.render_widget(List::new(list), layout[0]);
        drop(room);

        self.message_prompt.draw(frame, layout[1]);
    }

    #[expect(clippy::unwrap_used, reason = "not planned by trait")] // TODO: handle it
    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let key_event = event.as_key_press_event()?;

        if key_event.code.is_enter() {
            let message = self.message_prompt.take_value();
            let room = safe_unlock(&self.room).as_room();
            room.send_plain(&message).await.unwrap();
            return None;
        }

        self.message_prompt.on_event(event).await
    }
}
