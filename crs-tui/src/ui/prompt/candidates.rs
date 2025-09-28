//! Displays the selection part of the prompt

use core::convert::Infallible;
use core::fmt::Display;

use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::Paragraph;

use crate::ui::component::Component;

/// List of possible candidates to display
pub struct Candidates<T: Display> {
    /// List of all the possible candidates, whether they correspond to the
    /// search or not
    all:      Vec<T>,
    /// Currently selected item, if used with entries
    cursor:   Option<usize>,
    /// Display of the candidates that match the search
    matching: Vec<String>,
}

impl<T: Display> Candidates<T> {
    /// Decrement the cursor position after pressing tab with the new
    /// position, if it is valid.
    #[expect(clippy::arithmetic_side_effects, reason = "explicitly checked")]
    const fn cursor_decrement(&mut self) {
        if self.all.is_empty() {
            return;
        }
        self.cursor = Some(match self.cursor {
            None | Some(0) => self.all.len() - 1,
            Some(cursor) => cursor - 1,
        });
    }

    /// Increment the cursor position after pressing tab with the new
    /// position, if it is valid.
    const fn cursor_increment(&mut self) {
        if self.all.is_empty() {
            return;
        }
        self.cursor = Some(match self.cursor {
            None => 0,
            Some(cursor) => {
                let incremented = cursor.saturating_add(1);
                if incremented == self.all.len() { 0 } else { incremented }
            }
        });
    }

    /// Returns the number of candidates that match the search input
    pub const fn nb_matching(&self) -> usize {
        self.matching.len()
    }

    /// Returns a new empty [`Candidates`] with the given list of candidates
    pub const fn new(list: Vec<T>) -> Self {
        Self { cursor: None, all: list, matching: vec![] }
    }

    /// Returns the first possible entries that match the search
    pub fn update_matching(&mut self, input: &str) {
        self.matching = self
            .all
            .iter()
            .filter_map(|entry| {
                let formatted = format!("{entry}");
                formatted.contains(input).then_some(formatted)
            })
            .collect();
    }
}

impl<T: Display> Component for Candidates<T> {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut ratatui::Frame<'_>, area: Rect) {
        if self.matching.is_empty() {
            let empty = Text::from("No matching entry")
                .style(Style::new().fg(Color::Red))
                .centered();
            frame.render_widget(empty, area);
        }

        let lines = self
            .matching
            .iter()
            .map(|entry| Line::from(entry.as_str()))
            .collect::<Vec<_>>();

        frame.render_widget(Paragraph::new(lines), area);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        if let Some(key_event) = event.as_key_press_event() {
            if key_event.code.is_tab() {
                self.cursor_increment();
            } else if key_event.code.is_back_tab() {
                self.cursor_decrement();
            }
        }
        None
    }
}
