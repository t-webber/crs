//! Component that displays an input at the middle of the page

mod candidates;

use core::convert::Infallible;
use core::fmt::Display;

use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::Block;

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::prompt::candidates::Candidates;
use crate::ui::widgets::{grid_center, saturating_cast};

/// Struct to send the data after submission
pub struct PromptSubmit<T>(pub T);

/// Popup to create a room
pub struct Prompt<T: Display> {
    /// List of possible responses
    candidates: Candidates<T>,
    /// Input to enter the name of the room to be create
    input:      Input<'static>,
    /// Message to display in the prompt, including error messages and loading
    /// statuses
    message:    Status,
    /// Title of the prompt
    title:      &'static str,
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

    /// Create [`CreateRoom`] component
    pub const fn new(
        input: Input<'static>,
        title: &'static str,
        list: Vec<T>,
    ) -> Self {
        Self {
            input,
            title,
            message: Status::None,
            candidates: Candidates::new(list),
        }
    }
}

impl<T: Display> Component for Prompt<T> {
    type ResponseData = Status;
    type UpdateState = String;

    #[expect(clippy::arithmetic_side_effects, reason = "width >= 20")]
    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let width = (area.width - 2).min(50);

        let message = self.message.as_content();
        let message_height = message.map_or(0, |msg| {
            saturating_cast(msg.0.len()).saturating_div(width).saturating_add(1)
        });

        let input_height = self.input.height();

        let mut height = input_height + 4 + message_height;

        let max_matching_displayed = area.height.saturating_sub(height);
        let nb_matching = saturating_cast(self.candidates.nb_matching()).max(1);
        let candidates_height = nb_matching.min(max_matching_displayed);

        height += candidates_height;

        let popup_area = grid_center(
            Constraint::Length(width),
            Constraint::Length(height),
            area,
        );

        self.draw_border(frame, popup_area);

        let layout = Layout::new(Direction::Vertical, [
            Constraint::Length(input_height),
            Constraint::Length(message_height),
            Constraint::Length(candidates_height),
        ])
        .margin(2)
        .split(popup_area);

        self.input.draw(frame, layout[0]);

        if let Some((content, colour)) = message {
            let component =
                Text::from(content).style(Style::new().fg(colour)).centered();
            frame.render_widget(component, layout[1]);
        }

        self.candidates.draw(frame, layout[2]);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let code = event.as_key_press_event()?.code;
        if code.is_enter() {
            return Some(self.input.take_value());
        }
        if code.is_tab() || code.is_back_tab() {
            let _: Infallible = self.candidates.on_event(event).await?;
        } else {
            self.input.on_event(event).await;
            self.candidates.update_matching(self.input.as_value());
        }
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
