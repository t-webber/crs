//! Component that displays an input at the middle of the page

use core::convert::Infallible;

use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Rect};

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::widgets::grid_center;

/// Struct to update the contents of the error
pub struct ErrorMessage(pub String);

/// Struct to send the data after submission
pub struct PromptSubmit(pub String);

/// Popup to create a room
pub struct Prompt {
    /// Error to display in the prompt
    error: Option<String>,
    /// Input to enter the name of the room to be create
    input: Input<'static>,
    /// Title of the prompt
    title: &'static str,
}

impl Prompt {
    /// Create [`CreateRoom`] component
    pub const fn new(input: Input<'static>, title: &'static str) -> Self {
        Self { input, title, error: None }
    }
}

impl Component for Prompt {
    type ResponseData = ErrorMessage;
    type UpdateState = PromptSubmit;

    #[expect(clippy::arithmetic_side_effects, reason = "width >= 20")]
    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let height = self.input.height();
        let width = (area.width - 2).min(50);

        let popup_area = grid_center(
            Constraint::Length(width),
            Constraint::Length(height),
            area,
        );

        self.input.draw(frame, popup_area);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        if let Some(key_event) = event.as_key_press_event()
            && key_event.code.is_enter()
        {
            return Some(PromptSubmit(self.input.take_value()));
        }
        let _: Infallible = self.input.on_event(event).await?;
        None
    }

    fn update(&mut self, response_data: Self::ResponseData) {
        let ErrorMessage(error) = response_data;
        self.error = Some(error);
    }
}
