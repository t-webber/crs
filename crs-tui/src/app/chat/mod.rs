//! Main page displayed with the chats

mod conversation;
mod menu;

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;
use std::sync::Mutex;

use backend::room::DisplayRoom;
use backend::user::User;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::chat::conversation::Conversation;
use crate::app::chat::menu::RoomList;
use crate::ui::component::Component;
use crate::ui::widgets::{Instructions, InstructionsBuilder, linear_center};
use crate::utils::safe_unlock;

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct ChatPage {
    /// Open conversation
    conversation: Option<Conversation>,
    /// Menu with the list of rooms
    menu:         RoomList,
    /// Rooms visible by the user
    rooms:        Arc<Mutex<Vec<Arc<DisplayRoom>>>>,
    /// User to interact with matrix server
    user:         Arc<User>,
}

impl ChatPage {
    /// Create a new chat page with the given logged in user
    ///
    /// The rooms and their content will load in the background.
    pub fn new(user: Arc<User>) -> Self {
        let rooms = Arc::new(Mutex::new(vec![]));
        let rooms_adder = Arc::clone(&rooms);
        let user_adder = Arc::clone(&user);
        let _handle = tokio::spawn(async move {
            let on_room_load = move |room: DisplayRoom| {
                safe_unlock(&rooms_adder).push(Arc::new(room));
            };
            user_adder.load_rooms(on_room_load).await
        });
        let menu = RoomList::new(Arc::clone(&rooms));
        Self { rooms, user, menu, conversation: None }
    }
}

impl Component for ChatPage {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let layout = Layout::new(Direction::Horizontal, [
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

        self.menu.draw(frame, layout[0]);
        if let Some(conversation) = &self.conversation {
            conversation.draw(frame, layout[1]);
        } else {
            no_conversation(frame, layout[1]);
        }
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let index = self.menu.on_event(event).await?;
        self.conversation = Some(Conversation::new(Arc::clone(
            &safe_unlock(&self.rooms)[index],
        )));
        None
    }
}

/// Displays the message for when no chat is opened.
#[expect(
    clippy::integer_division,
    clippy::integer_division_remainder_used,
    clippy::arithmetic_side_effects,
    reason = "want rounded value"
)]
pub fn no_conversation(frame: &mut Frame<'_>, area: Rect) {
    let Instructions { line, width } = InstructionsBuilder::default()
        .text(" Use")
        .key("Up")
        .text("and")
        .key("Down")
        .text("to find the conversation, then")
        .key("Enter")
        .text("to open it here. ")
        .build();

    let height = (width / area.width).saturating_add(1);

    let center =
        linear_center(Constraint::Length(height), Direction::Vertical, area);

    let paragraph = Paragraph::new(line)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, center);
}
