//! Selector to choose which menu to open

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;
use std::sync::Mutex;

use backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, List, ListItem, Paragraph, Wrap};

use crate::ui::component::Component;
use crate::ui::widgets::{Instructions, InstructionsBuilder, grid_center};
use crate::utils::safe_unlock;

/// Message displayed when no rooms are accessible from the user.
const EMPTY_MSG: &str = "You don't have access to any rooms. Connect a bridge \
                         or create a new chat.";

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct RoomList {
    /// Rooms visible by the user
    rooms:         Arc<Mutex<Vec<Arc<DisplayRoom>>>>,
    /// Room selected on the side bar with the list of chats.
    ///
    /// Press enter to open this room in the chat panel, and use arrows to
    /// selected another room.
    selected_room: usize,
}

impl RoomList {
    /// Draw the room list for when no rooms are available.
    #[expect(
        clippy::integer_division,
        clippy::integer_division_remainder_used,
        clippy::arithmetic_side_effects,
        reason = "want rounded value"
    )]
    fn draw_empty(frame: &mut Frame<'_>, area: Rect) {
        let rect_width = area.width - 4;

        let Instructions { line, width } = Self::instructions();
        let height = (width / rect_width).saturating_add(1);

        let paragraph = Paragraph::new(line)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        let rect = grid_center(
            Constraint::Length(rect_width),
            Constraint::Length(height),
            area,
        );
        frame.render_widget(paragraph, rect);
    }

    /// Instructions to be displayed when no rooms are accessible from the user.
    fn instructions() -> Instructions<'static> {
        InstructionsBuilder::default()
            .text("You don't have access to any rooms. Press")
            .key("C-n")
            .text("to create a new chat or")
            .key("C-b")
            .text("to connect a bridge.")
            .build()
    }

    /// Create a new menu list with the same rooms than the chat page.
    ///
    /// The rooms and their content are loaded by the chat page in the backend.
    pub const fn new(rooms: Arc<Mutex<Vec<Arc<DisplayRoom>>>>) -> Self {
        Self { rooms, selected_room: 0 }
    }
}

impl Component for RoomList {
    type ResponseData = Infallible;
    type UpdateState = usize;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        debug_assert!(
            area.as_size().width >= 10,
            "menu shouldn't be displayed on small screens"
        );
        let unknown = String::from("<unknown>");

        if safe_unlock(&self.rooms).is_empty() {
            Self::draw_empty(frame, area);
        }

        let name_list = safe_unlock(&self.rooms)
            .iter()
            .enumerate()
            .map(|(idx, room)| {
                let name = room.as_name().unwrap_or(&unknown).as_str();
                if idx == self.selected_room {
                    ListItem::new(format!(">{name}",))
                        .style(Style::new().fg(Color::Green))
                } else {
                    ListItem::new(format!(" {name}",))
                }
            })
            .collect::<Vec<_>>();

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
                if new_index < safe_unlock(&self.rooms).len() {
                    self.selected_room = new_index;
                }
            }
            KeyCode::Enter => return Some(self.selected_room),
            _ => (),
        }
        None
    }
}
