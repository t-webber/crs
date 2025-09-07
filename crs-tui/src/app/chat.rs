//! Main page displayed with the chats

extern crate alloc;
use alloc::sync::Arc;

use backend::room::DisplayRoom;
use backend::user::User;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{List, ListItem};

use crate::ui::component::Component;

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct ChatPage {
    /// Room currently opened in the chat panel
    current_room: usize,
    /// Rooms visible by the user
    rooms:        Vec<DisplayRoom>,
    /// User to interact with matrix server
    user:         Arc<User>,
}

impl ChatPage {
    /// Create a new chat page with the given logged in user
    pub async fn new(user: Arc<User>) -> Self {
        let rooms = user.list_rooms().await;
        Self { rooms, user, current_room: 0 }
    }
}

impl Component for ChatPage {
    type ResponseData = ();
    type UpdateState = ();

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let unknown = String::from("<unknown>");

        let name_list = self
            .rooms
            .iter()
            .enumerate()
            .map(|(idx, room)| {
                let name = room.as_name().as_ref().unwrap_or(&unknown).as_str();
                if idx == self.current_room {
                    ListItem::new(format!(">{name}",))
                        .style(Style::new().fg(Color::Green))
                } else {
                    ListItem::new(format!(" {name}",))
                }
            })
            .collect::<Vec<_>>();

        let list = List::new(name_list);

        frame.render_widget(list, area);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let Event::Key(key_event) = event else {
            return None;
        };
        if key_event.kind != KeyEventKind::Press {
            return None;
        }
        match key_event.code {
            KeyCode::Up =>
                self.current_room = self.current_room.saturating_sub(1),
            KeyCode::Down => {
                let new_index = self.current_room.saturating_add(1);
                if new_index < self.rooms.len() {
                    self.current_room = new_index;
                }
            }
            _ => (),
        }
        None
    }
}

/// Room displayed in the list of rooms
pub struct Room {
    /// Id of the room
    id:   String,
    /// Name of the room
    name: String,
}
