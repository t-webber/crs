//! Popup to search and select a room

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;
use std::sync::Mutex;

use crs_backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::widgets::{InstructionsBuilder, grid_center, saturating_cast};
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
    results: Vec<(usize, Arc<str>)>,
    /// List of all the loaded rooms
    rooms:   Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>,
}

impl RoomSearch {
    /// Decrement the cursor position after pressing tab with the new
    /// position, if it is valid.
    #[expect(clippy::arithmetic_side_effects, reason = "explicitly checked")]
    const fn cursor_decrement(&mut self) {
        if self.results.is_empty() {
            return;
        }
        self.cursor = Some(match self.cursor {
            None | Some(0) => self.results.len() - 1,
            Some(cursor) => cursor - 1,
        });
    }

    /// Increment the cursor position after pressing tab with the new
    /// position, if it is valid.
    const fn cursor_increment(&mut self) {
        if self.results.is_empty() {
            return;
        }
        self.cursor = Some(match self.cursor {
            None => 0,
            Some(cursor) => {
                let incremented = cursor.saturating_add(1);
                if incremented == self.results.len() { 0 } else { incremented }
            }
        });
    }

    /// Draws the popup borders with the titles and keymap instructions
    fn draw_popup(frame: &mut Frame<'_>, area: Rect) {
        let title = Line::from(" Search room ").centered();

        let instructions = InstructionsBuilder::default()
            .text(" Select")
            .key("Tab/backtab")
            .text("Open")
            .key("Enter")
            .build();

        frame.render_widget(
            Block::bordered()
                .title(title)
                .title_bottom(instructions.line.centered()),
            area,
        );
    }

    /// Draws the line of results
    fn draw_results(&self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .results
            .iter()
            .enumerate()
            .map(|(idx, (_, name))| {
                if Some(idx) == self.cursor {
                    Line::from(format!("*{name}*")).style(Color::Green)
                } else {
                    Line::from(format!(" {name} "))
                }
            })
            .collect::<Vec<_>>();

        frame.render_widget(Paragraph::new(lines), area);
    }

    /// Creates a new [`RoomSearch`].
    pub const fn new(rooms: Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>) -> Self {
        Self {
            input: Input::new().with_active(),
            rooms,
            cursor: None,
            results: vec![],
        }
    }

    /// Refreshes the results after changing the search query.
    fn refresh_results(&mut self) {
        self.results.clear();
        let rooms = safe_unlock(&self.rooms);
        for (index, room) in rooms.iter().enumerate() {
            if let Ok(name) = safe_unlock(room).as_name()
                && name.contains(self.input.as_value())
            {
                self.results.push((index, name));
                if self.results.len() >= 20 {
                    return;
                }
            }
        }
    }
}

impl Component for RoomSearch {
    type ResponseData = Infallible;
    type UpdateState = Arc<Mutex<DisplayRoom>>;

    #[expect(clippy::arithmetic_side_effects, reason = "width >= 20")]
    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let width = area.width - 2;
        let height = saturating_cast(self.results.len())
            .saturating_add(Input::HEIGHT_WITHOUT_LABEL)
            .saturating_add(4);

        let popup = grid_center(
            Constraint::Length(width),
            Constraint::Length(height),
            area,
        );

        Self::draw_popup(frame, popup);

        let layout = Layout::new(Direction::Vertical, [
            Constraint::Length(Input::HEIGHT_WITHOUT_LABEL),
            Constraint::Fill(1),
        ])
        .margin(2)
        .split(popup);

        self.input.draw(frame, layout[0]);
        self.draw_results(frame, layout[1]);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let key_event = event.as_key_press_event()?;
        match key_event.code {
            KeyCode::Tab => self.cursor_increment(),
            KeyCode::BackTab => self.cursor_decrement(),
            KeyCode::Enter =>
                if let Some(cursor) = self.cursor {
                    let real_index = self.results[cursor].0;
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
