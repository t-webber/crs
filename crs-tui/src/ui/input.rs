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
#[derive(Default)]
pub struct Input<'label> {
    /// Has error
    has_error: bool,
    /// Whether pressing char keys edits this input or not
    is_active: bool,
    /// Whether to hide the content of the input or not
    is_hidden: bool,
    /// Label to be placed on top of the input
    label:     Option<&'label str>,
    /// Value inside the input
    value:     String,
}

impl<'label> Input<'label> {
    /// Number of lines to ask to render an input
    pub const HEIGHT_WITHOUT_LABEL: u16 = 3;
    /// Number of lines to ask to render an input
    pub const HEIGHT_WITH_LABEL: u16 = 5;

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

    /// Draw the input box with the value inside
    fn draw_value(&self, frame: &mut Frame<'_>, area: Rect) {
        let value = Paragraph::new(if self.is_hidden {
            let hidden_value: String =
                repeat_n('*', self.value.len()).collect();
            Text::from(hidden_value)
        } else {
            Text::from(self.value.as_str())
        })
        .block(Block::bordered().border_style(self.border_style()));

        frame.render_widget(value, area);
    }

    /// Returns the value of the input
    pub fn into_value(self) -> String {
        self.value
    }

    /// Checks if the value is empty
    pub const fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Creates a default inactive visible empty input
    pub const fn new() -> Self {
        Input {
            has_error: false,
            is_active: false,
            is_hidden: false,
            label:     None,
            value:     String::new(),
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

    /// Mark the input as active
    pub const fn with_active(mut self) -> Self {
        self.is_active = true;
        self
    }

    /// Hide the value of the input with stars
    pub const fn with_hidden(mut self) -> Self {
        self.is_hidden = true;
        self
    }

    /// Add a label to the input
    pub const fn with_label(mut self, label: &'label str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the initial value of the input
    pub fn with_value(mut self, value: String) -> Self {
        self.value = value;
        self
    }
}

impl Component for Input<'_> {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        if let Some(label) = self.label {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(3)])
                .split(area);
            frame.render_widget(Text::from(label), layout[0]);
            self.draw_value(frame, layout[1]);
        } else {
            self.draw_value(frame, area);
        }
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
