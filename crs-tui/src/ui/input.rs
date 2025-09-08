//! Reusable input component

use core::convert::Infallible;
use core::iter::repeat_n;

use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Paragraph};

use crate::ui::component::Component;

/// Bordered input component
pub struct Input {
    /// Has error
    has_error: bool,
    /// Whether pressing char keys edits this input or not
    is_active: bool,
    /// Whether to hide the content of the input or not
    is_hidden: bool,
    /// Label to be placed on top of the input
    label:     String,
    /// Value inside the input
    value:     String,
}

impl Input {
    /// Number of lines to ask to render an input
    pub const HEIGHT: u16 = 5;

    /// Border colour
    fn border_style(&self) -> Style {
        Style::default().fg(if self.has_error {
            Color::Red
        } else if self.is_active {
            Color::Green
        } else {
            Color::Reset
        })
    }

    /// Returns the value of the input
    pub fn into_value(self) -> String {
        self.value
    }

    /// Checks if the value is empty
    pub const fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Create a new input
    pub const fn new(label: String, is_active: bool, is_hidden: bool) -> Self {
        Self {
            is_hidden,
            is_active,
            label,
            has_error: false,
            value: String::new(),
        }
    }

    /// Focus the input and capture the keys
    pub const fn set_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    /// Sets the inner value of the input
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl Component for Input {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ])
            .split(area);

        let label = Text::from(self.label.as_str());
        let value = Paragraph::new(if self.is_hidden {
            let hidden_value: String =
                repeat_n('*', self.value.len()).collect();
            Text::from(hidden_value)
        } else {
            Text::from(self.value.as_str())
        })
        .block(Block::bordered().border_style(self.border_style()));

        frame.render_widget(&label, layout[1]);
        frame.render_widget(&value, layout[2]);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let Event::Key(key_event) = event else {
            return None;
        };
        if key_event.kind != KeyEventKind::Press {
            return None;
        }
        match key_event.code {
            KeyCode::Char(ch) => self.value.push(ch),
            KeyCode::Backspace => {
                self.value.pop();
            }
            _ => (),
        }

        None
    }
}
