//! Component that displays an input at the middle of the page

use core::convert::Infallible;
mod entries;
use core::fmt::Display;

use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph};

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::widgets::{grid_center, saturating_cast};

/// Struct to send the data after submission
pub struct PromptSubmit<T>(pub T);

/// Popup to create a room
pub struct Prompt<T: Display> {
    /// Currently selected item, if used with entries
    cursor:  Option<usize>,
    /// List of possible responses
    entries: Option<Vec<T>>,
    /// Input to enter the name of the room to be create
    input:   Input<'static>,
    /// Message to display in the prompt, including error messages and loading
    /// statuses
    message: Status,
    /// Title of the prompt
    title:   &'static str,
}

impl<T: Display> Prompt<T> {
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

    /// Draws the border around the prompt
    fn draw_border(&self, frame: &mut Frame<'_>, popup_area: Rect) {
        let block = Block::bordered()
            .title(self.title)
            .title_alignment(Alignment::Center)
            .title_style(Style::new().fg(Color::Yellow));
        frame.render_widget(block, popup_area);
    }

    /// Returns the first possible entries that match the search
    fn get_possibilites(&self, max_number: usize) -> Option<Vec<String>> {
        let value = self.input.as_value();
        let entries = self.entries.as_ref()?;
        Some(
            entries
                .iter()
                .filter_map(|entry| {
                    let formatted = format!("{entry}");
                    formatted.contains(value).then_some(formatted)
                })
                .take(max_number)
                .collect(),
        )
    }

    /// Create [`CreateRoom`] component
    pub const fn new(input: Input<'static>, title: &'static str) -> Self {
        Self {
            input,
            title,
            message: Status::None,
            entries: None,
            cursor: None,
        }
    }

    /// Create [`CreateRoom`] component
    pub const fn new_with_list(
        input: Input<'static>,
        title: &'static str,
        entries: Vec<T>,
    ) -> Self {
        Self {
            input,
            title,
            message: Status::None,
            entries: Some(entries),
            cursor: None,
        }
    }
}

impl<T: Display> Component for Prompt<T> {
    type ResponseData = Status;
    type UpdateState = T;

    #[expect(clippy::arithmetic_side_effects, reason = "width >= 20")]
    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let width = (area.width - 2).min(50);

        let message = self.message.as_content();
        let message_height = message.map_or(0, |msg| {
            saturating_cast(msg.0.len()).saturating_div(width).saturating_add(1)
        });

        let input_height = self.input.height();

        let mut height = input_height + 4 + message_height;

        let max_possibilities_nb = area.height.saturating_sub(height);
        let possibilities = self.get_possibilites(max_possibilities_nb.into());

        let possibilities_height = possibilities
            .as_ref()
            .map_or(0, |possib| saturating_cast(possib.len()));

        height += possibilities_height;

        let popup_area = grid_center(
            Constraint::Length(width),
            Constraint::Length(height),
            area,
        );

        self.draw_border(frame, popup_area);

        let layout = Layout::new(Direction::Vertical, [
            Constraint::Length(input_height),
            Constraint::Length(message_height),
            Constraint::Length(possibilities_height),
        ])
        .margin(2)
        .split(popup_area);

        self.input.draw(frame, layout[0]);

        if let Some((content, colour)) = message {
            let component =
                Text::from(content).style(Style::new().fg(colour)).centered();
            frame.render_widget(component, layout[1]);
        }

        if let Some(possible_entries) = possibilities {
            draw_possibilities(frame, layout[2], &possible_entries);
        }
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        if let Some(key_event) = event.as_key_press_event() {
            if key_event.code.is_enter() {
                todo!()
            }
            if key_event.code.is_up() {}
        };
        let _: Infallible = self.input.on_event(event).await?;
        None
    }

    fn update(&mut self, response_data: Self::ResponseData) {
        self.message = response_data;
        self.input.set_active(!matches!(self.message, Status::Submitting));
    }
}

/// Status information to inform the user on
pub enum Status {
    /// Error message to display
    Error(String),
    /// Nothing to display
    None,
    /// Submition in progress
    Submitting,
}

impl Status {
    /// Returns the message to display, as well as the colour if it has a
    /// mesaage.
    const fn as_content(&self) -> Option<(&str, Color)> {
        match self {
            Self::None => None,
            Self::Error(message) => Some((message.as_str(), Color::Red)),
            Self::Submitting => Some(("Submitting...", Color::Green)),
        }
    }
}

/// Draws the list of matching entries
fn draw_possibilities(
    frame: &mut Frame<'_>,
    area: Rect,
    possible_entries: &[String],
) {
    if possible_entries.is_empty() {
        let empty = Text::from("No corresponding entry")
            .style(Style::new().fg(Color::Red))
            .centered();
        frame.render_widget(empty, area);
    }

    let lines = possible_entries
        .iter()
        .map(|entry| Line::from(entry.as_str()))
        .collect::<Vec<_>>();

    frame.render_widget(Paragraph::new(lines), area);
}
