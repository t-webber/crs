//! Selector to choose which menu to open

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;
use std::sync::{LazyLock, Mutex};

use crs_backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, List, ListItem, Paragraph, Wrap};

use crate::ui::component::Component;
use crate::ui::widgets::{
    Instructions, InstructionsBuilder, fully_centred_content, grid_centre, saturating_cast
};
use crate::utils::{UNKNOWN_NAME, safe_unlock};

/// Instructions to display keymaps
static INSTRUCTIONS: LazyLock<Instructions<'static>> = LazyLock::new(|| {
    InstructionsBuilder::default()
        .text(" Select")
        .key("Up/Down/Right")
        .text("Search")
        .key("C-k")
        .build()
});

/// Minimum width to display the room list
pub static ROOM_LIST_WIDTH: LazyLock<u16> =
    LazyLock::new(|| INSTRUCTIONS.width.saturating_add(2));

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct RoomList {
    /// Indicates whether the rooms are still loading
    ///
    /// This is used to determine if an empty list of rooms should be
    /// interpreted as "They are not accessible yet" or "There aren't any".
    is_loading:    bool,
    /// Rooms visible by the user
    rooms:         Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>,
    /// Room selected on the side bar with the list of chats.
    ///
    /// Press enter to open this room in the chat panel, and use arrows to
    /// selected another room.
    selected_room: usize,
}

impl RoomList {
    /// Draws the room list for when no rooms are available.
    #[expect(clippy::arithmetic_side_effects, reason = "width >= 20")]
    fn draw_empty(frame: &mut Frame<'_>, area: Rect) {
        debug_assert!(
            area.as_size().width >= 20,
            "menu shouldn't be displayed on small screens"
        );

        let instructions = Self::instructions();

        let rect =
            fully_centred_content(instructions.width, area.width - 4, area);

        let paragraph = Paragraph::new(instructions.line)
            .wrap(Wrap { trim: true })
            .centered();

        frame.render_widget(paragraph, rect);
    }

    /// Draws the loading message until the rooms are fetched from the mautrix
    fn draw_loading(frame: &mut Frame<'_>, area: Rect) {
        let text = "Loading...";

        let rect = grid_centre(
            Constraint::Length(saturating_cast(text.len())),
            Constraint::Length(1),
            area,
        );

        frame.render_widget(Text::from("Loading..."), rect);
    }

    /// Draws the list of rooms to select which room to open in the conversation
    /// part of the window.
    fn draw_room_list(&self, frame: &mut Frame<'_>, area: Rect) {
        let (start, current_index, stop) = self.get_section_delimitations(area);

        let name_list = safe_unlock(&self.rooms)[start..stop]
            .iter()
            .enumerate()
            .map(|(idx, room)| {
                let room_locked = safe_unlock(room);
                let name = room_locked
                    .as_name()
                    .unwrap_or_else(|| UNKNOWN_NAME.clone());
                let (text, colour) = if idx == current_index {
                    (format!(">{name}"), Color::Green)
                } else {
                    (format!(" {name}"), Color::Reset)
                };
                drop(room_locked);
                ListItem::new(text).style(Style::new().fg(colour))
            })
            .collect::<Vec<_>>();

        let block = if area.width >= *ROOM_LIST_WIDTH {
            Block::bordered()
                .border_style(Style::default().fg(Color::Gray))
                .title_bottom(INSTRUCTIONS.line.clone())
        } else {
            Block::bordered().border_style(Color::Gray)
        };

        let list = List::new(name_list).block(block);

        frame.render_widget(list, area);
    }

    /// Marks the loading as ended
    pub const fn end_loading(&mut self) {
        self.is_loading = false;
    }

    /// Returns the indices to slice the rooms displayed in the area
    ///
    /// # Returns
    ///
    /// A tuple with (start, current, end).
    ///
    /// - start: Index of the first room to display
    /// - current: relative index within the displayed slice of the currently
    ///   selected room
    /// - end: Index to stop the slice
    #[expect(clippy::arithmetic_side_effects, reason = "checked")]
    fn get_section_delimitations(&self, area: Rect) -> (usize, usize, usize) {
        let nb_rooms = safe_unlock(&self.rooms).len();
        let current_index = self.selected_room;

        let nb_rooms_displayed = usize::from(area.height - 2);

        let limit = nb_rooms_displayed >> 1_u32;

        let (start, stop) = if current_index <= limit {
            (0, nb_rooms_displayed.min(nb_rooms))
        } else if current_index >= nb_rooms - limit {
            (nb_rooms.saturating_sub(nb_rooms_displayed), nb_rooms)
        } else {
            (current_index - limit, current_index + limit)
        };

        (start, current_index - start, stop)
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
    pub const fn new(rooms: Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>) -> Self {
        Self { rooms, selected_room: 0, is_loading: true }
    }
}

impl Component for RoomList {
    type ResponseData = Infallible;
    type UpdateState = usize;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        if safe_unlock(&self.rooms).is_empty() {
            if self.is_loading {
                Self::draw_loading(frame, area);
            } else {
                Self::draw_empty(frame, area);
            }
        } else {
            self.draw_room_list(frame, area);
        }
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let key_event = event.as_key_press_event()?;
        match key_event.code {
            KeyCode::Up =>
                self.selected_room = self.selected_room.saturating_sub(1),
            KeyCode::Down => {
                let len = safe_unlock(&self.rooms).len();
                let new_index = self.selected_room.saturating_add(1);
                if new_index < len {
                    self.selected_room = new_index;
                }
            }
            KeyCode::Right => return Some(self.selected_room),
            _ => (),
        }
        None
    }
}
