//! Main page displayed with the chats

extern crate alloc;
use alloc::sync::Arc;
use std::sync::Mutex;

use backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, List, ListItem};

use crate::ui::component::Component;

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct RoomList {
    /// Rooms visible by the user
    rooms:         Arc<Mutex<Vec<DisplayRoom>>>,
    /// Room selected on the side bar with the list of chats.
    ///
    /// Press enter to open this room in the chat panel, and use arrows to
    /// selected another room.
    selected_room: usize,
}

impl RoomList {
    /// Create a new chat page with the given logged in user
    ///
    /// The rooms and their content will load in the background.
    pub const fn new(rooms: Arc<Mutex<Vec<DisplayRoom>>>) -> Self {
        Self { rooms, selected_room: 0 }
    }
}

impl Component for RoomList {
    type ResponseData = ();
    type UpdateState = usize;

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let unknown = String::from("<unknown>");

        let name_list = if self.rooms.lock().unwrap().is_empty() {
            vec![ListItem::new("no rooms")]
        } else {
            self.rooms
                .lock()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(idx, room)| {
                    let name =
                        room.as_name().as_ref().unwrap_or(&unknown).as_str();
                    if idx == self.selected_room {
                        ListItem::new(format!(">{name}",))
                            .style(Style::new().fg(Color::Green))
                    } else {
                        ListItem::new(format!(" {name}",))
                    }
                })
                .collect::<Vec<_>>()
        };

        let list = List::new(name_list).block(
            Block::bordered().border_style(Style::default().fg(Color::Gray)),
        );

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
                self.selected_room = self.selected_room.saturating_sub(1),
            KeyCode::Down => {
                let new_index = self.selected_room.saturating_add(1);
                if new_index < self.rooms.lock().unwrap().len() {
                    self.selected_room = new_index;
                }
            }
            KeyCode::Enter => return Some(self.selected_room),
            _ => (),
        }
        None
    }
}
