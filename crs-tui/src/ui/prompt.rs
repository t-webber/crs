//! Component that displays an input at the middle of the page

use core::convert::Infallible;

use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::Block;

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::widgets::{grid_center, saturating_cast};

/// Struct to send the data after submission
pub struct PromptSubmit<T>(pub T);

/// Popup to create a room
pub struct Prompt<T> {
    /// Input to enter the name of the room to be create
    input:            Input<'static>,
    /// Message to display in the prompt, including error messages and loading
    /// statuses
    message:          Status,
    /// List of possible responses
    possible_entries: Option<Vec<T>>,
    /// Title of the prompt
    title:            &'static str,
}

impl<T> Prompt<T> {
    /// Create [`CreateRoom`] component
    pub const fn new(input: Input<'static>, title: &'static str) -> Self {
        Self { input, title, message: Status::None, possible_entries: None }
    }

    /// Create [`CreateRoom`] component
    pub const fn new_with_list(
        input: Input<'static>,
        title: &'static str,
        possible_entries: Vec<T>,
    ) -> Self {
        Self {
            input,
            title,
            message: Status::None,
            possible_entries: Some(possible_entries),
        }
    }

    /// Render the prompt on the screen
    #[expect(clippy::arithmetic_side_effects, reason = "width >= 20")]
    fn draw_prompt(&self, frame: &mut Frame<'_>, area: Rect) {
        let width = (area.width - 2).min(50);

        let error_height = self.message.as_content().map_or(0, |error| {
            saturating_cast(error.0.len())
                .saturating_div(width)
                .saturating_add(1)
        });

        let input_height = self.input.height();

        let height = input_height + 4 + error_height;

        let popup_area = grid_center(
            Constraint::Length(width),
            Constraint::Length(height),
            area,
        );

        let block = Block::bordered()
            .title(self.title)
            .title_alignment(Alignment::Center)
            .title_style(Style::new().fg(Color::Yellow));
        frame.render_widget(block, popup_area);

        let layout = Layout::new(Direction::Vertical, [
            Constraint::Length(input_height),
            Constraint::Length(error_height),
        ])
        .margin(2)
        .split(popup_area);

        self.input.draw(frame, layout[0]);

        if let Some((error_message, colour)) = self.message.as_content() {
            let error_component = Text::from(error_message)
                .style(Style::new().fg(colour))
                .centered();
            frame.render_widget(error_component, layout[1]);
        }
    }

    /// Updates the status of the prompt, to display loading and error
    /// information
    fn update_status(&mut self, status: Status) {
        self.message = status;
        if matches!(self.message, Status::Submitting) {
            self.input.set_active(false);
        } else {
            self.input.set_active(true);
        }
    }
}

impl<T> Component for Prompt<T> {
    type ResponseData = Status;
    type UpdateState = T;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        self.draw_prompt(frame, area);
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
