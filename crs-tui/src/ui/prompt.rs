//! Component that displays an input at the middle of the page

use core::convert::Infallible;
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
        Self { input, title, message: Status::None, entries: None }
    }

    /// Create [`CreateRoom`] component
    pub const fn new_with_list(
        input: Input<'static>,
        title: &'static str,
        entries: Vec<T>,
    ) -> Self {
        Self { input, title, message: Status::None, entries: Some(entries) }
    }

    /// Updates the status of the prompt, to display loading and error
    /// information
    fn update_status(&mut self, status: Status) {
        self.message = status;
        self.input.set_active(!matches!(self.message, Status::Submitting));
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
        if let Some(key_event) = event.as_key_press_event()
            && key_event.code.is_enter()
        {
            todo!()
        } else {
            let _: Infallible = self.input.on_event(event).await?;
            None
        }
    }

    fn update(&mut self, response_data: Self::ResponseData) {
        self.update_status(response_data);
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
