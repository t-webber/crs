//! Main page displayed with the chats

mod conversation;
mod menu;

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;
use core::time::Duration;
use std::sync::Mutex;
use std::thread;

use crs_backend::room::DisplayRoom;
use crs_backend::user::User;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::chat::conversation::Conversation;
use crate::app::chat::menu::RoomList;
use crate::ui::component::Component;
use crate::ui::widgets::{InstructionsBuilder, fully_centered_content};
use crate::utils::safe_unlock;

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct ChatPage {
    /// Currently opened conversation
    conversation: Option<Conversation>,
    /// Menu with the list of rooms
    menu:         RoomList,
    /// Rooms visible by the user
    rooms:        Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>,
    /// User to interact with matrix server
    user:         Arc<User>,
}

impl ChatPage {
    /// Create a new chat page with the given logged in user
    ///
    /// The rooms and their content will load in the background.
    pub fn new(user: Arc<User>) -> Self {
        let rooms = Arc::new(Mutex::new(vec![]));
        let menu = RoomList::new(Arc::clone(&rooms));
        let mut this = Self { rooms, user, menu, conversation: None };
        this.synchronise_rooms();
        this
    }

    /// Synchronise the existing rooms, including name and messages
    fn synchronise_rooms(&mut self) {
        let rooms = Arc::clone(&self.rooms);
        let user = Arc::clone(&self.user);
        user.wait_for_visible_room();
        self.menu.end_loading();
        let _handle = tokio::spawn(async move {
            loop {
                let local_rooms = Arc::clone(&rooms);
                let on_room_load = move |new_room: DisplayRoom| {
                    let new_room_id = new_room.id();
                    if let Some(old_room) = safe_unlock(&local_rooms)
                        .iter_mut()
                        .find(|room| safe_unlock(room).id() == new_room_id)
                    {
                        safe_unlock(old_room).update_from(new_room);
                    } else {
                        safe_unlock(&local_rooms)
                            .push(Arc::new(Mutex::new(new_room)));
                    }
                };
                user.load_rooms(on_room_load).await.unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
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
        match self.menu.on_event(event.clone()).await {
            Some(index) => {
                let new_room = Arc::clone(&safe_unlock(&self.rooms)[index]);
                self.conversation = Some(Conversation::new(new_room));
            }
            None =>
                if let Some(conversation) = &mut self.conversation {
                    conversation.on_event(event).await;
                },
        }
        None
    }
}

/// Displays the message for when no chat is opened.
pub fn no_conversation(frame: &mut Frame<'_>, area: Rect) {
    let instructions = InstructionsBuilder::default()
        .text(" Use")
        .key("Up")
        .text("and")
        .key("Down")
        .text("to find the conversation, then")
        .key("Right")
        .text("to open it here. ")
        .build();

    let rect = fully_centered_content(instructions.width, area.width, area);

    let paragraph = Paragraph::new(instructions.line)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, rect);
}
