//! Popup to search and select a room

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;
use std::sync::Mutex;

use crs_backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::utils::safe_unlock;

/// Component to search a room by name.
pub struct RoomSearch {
    /// Current position of the cursor
    cursor:  Option<usize>,
    /// Input component to type the name of the room.
    input:   Input<'static>,
    /// First results corresponding to the search
    ///
    /// This only contains the indices of the rooms in the `rooms` field.
    results: Vec<usize>,
    /// List of all the loaded rooms
    rooms:   Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>,
}

impl RoomSearch {
    /// Create a new [`RoomSearch`].
    pub const fn new(rooms: Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>) -> Self {
        Self {
            input: Input::new().with_active().with_label("Room name"),
            rooms,
            cursor: None,
            results: vec![],
        }
    }

    /// Refresh the results after changing the search query.
    fn refresh_results(&mut self) {
        self.results.clear();
        let rooms = safe_unlock(&self.rooms);
        for (index, room) in rooms.iter().enumerate() {
            if let Ok(name) = safe_unlock(room).as_name()
                && name.contains(self.input.as_value())
            {
                self.results.push(index);
                if self.results.len() >= 10 {
                    return;
                }
            }
        }
    }

    /// Update the cursor after pressing tab/backtab with the new position, if
    /// it is valid.
    const fn update_cursor(&mut self, new_cursor: usize) {
        if new_cursor <= self.results.len() {
            self.cursor = Some(new_cursor);
        }
    }
}

impl Component for RoomSearch {
    type ResponseData = Infallible;
    type UpdateState = Arc<Mutex<DisplayRoom>>;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let layout = Layout::new(Direction::Vertical, [
            Constraint::Length(Input::HEIGHT_WITH_LABEL),
            Constraint::Fill(1),
        ])
        .split(area);

        self.input.draw(frame, layout[0]);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let key_event = event.as_key_press_event()?;
        match key_event.code {
            KeyCode::Tab => self.update_cursor(
                self.cursor.unwrap_or_default().saturating_add(1),
            ),
            KeyCode::BackTab => self.update_cursor(
                self.cursor.unwrap_or_default().saturating_sub(1),
            ),
            KeyCode::Enter =>
                if let Some(cursor) = self.cursor {
                    let real_index = self.results[cursor];
                    let rooms = safe_unlock(&self.rooms);
                    return Some(Arc::clone(&rooms[real_index]));
                },

            _ => {
                self.cursor = None;
                self.input.on_event(event).await;
                self.refresh_results();
                return None;
            }
        }
        None
    }
}
