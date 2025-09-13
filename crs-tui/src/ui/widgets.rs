//! Module that contains reusable widgets and UI elements for different screens
//! and components

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Stylize as _;
use ratatui::text::{Line, Span};

/// Widget to display instructions on the screen
pub struct Instructions<'text> {
    /// widget line
    pub line:  Line<'text>,
    /// space taken by the widget, for usage in constraints, layouts, etc.
    pub width: u16,
}

/// Widget to display keymap instructions on the screen.
///
/// # Examples
///
/// To display "Press <Enter> to open., use this:
///
/// ```rust
/// use crate::ui::widget::*;
///
/// let Instructions { line, width } = InstructionsBuilder::default()
///     .text("Press ")
///     .key("Enter")
///     .text(" to open.")
///     .build();
/// ```
#[derive(Default)]
pub struct InstructionsBuilder<'text> {
    /// Elements of the line to be displayed, texts and keys.
    elements: Vec<Span<'text>>,
    /// Total width of the final text
    width:    u16,
}

impl<'text> InstructionsBuilder<'text> {
    /// Build the widget from the provided list of text and keys.
    pub fn build(self) -> Instructions<'text> {
        Instructions { line: Line::from(self.elements), width: self.width }
    }

    /// Add a key part.
    pub fn key(mut self, key: &'text str) -> Self {
        self.elements.push(format!(" <{key}> ").yellow().bold());
        self.width = self
            .width
            .saturating_add(saturating_cast(key.len()))
            .saturating_add(4);
        self
    }

    /// Add a text part.
    ///
    /// Remember to add spaces.
    ///
    /// Also, keep it short or else it won't fit.
    pub fn text(mut self, text: &'text str) -> Self {
        self.elements.push(text.into());
        self.width = self.width.saturating_add(saturating_cast(text.len()));
        self
    }
}

/// Cast a [`usize`] to [`u16`] with saturation
///
/// If the input is too large to fit in a [`u16`]
/// [`u16::MAX`] is used
pub fn saturating_cast(value: usize) -> u16 {
    u16::try_from(value).unwrap_or(u16::MAX)
}

/// Create a centered frame to display data in a centered rectangle inside the
/// current page.
pub fn grid_center(
    horizontal_constraint: Constraint,
    vertical_constrait: Constraint,
    area: Rect,
) -> Rect {
    let horizontal_center =
        linear_center(horizontal_constraint, Direction::Horizontal, area);

    linear_center(vertical_constrait, Direction::Vertical, horizontal_center)
}

/// Create a unidimensionally centered frame to display data in the middle in
/// one [`Direction`]
pub fn linear_center(
    middle_constraint: Constraint,
    direction: Direction,
    area: Rect,
) -> Rect {
    Layout::new(direction, [
        Constraint::Fill(1),
        middle_constraint,
        Constraint::Fill(1),
    ])
    .split(area)[1]
}

/// Center text both vertically and horizontally, adapting the constraints to
/// the length of the text and the width of the area.
#[expect(clippy::arithmetic_side_effects, reason = "round value")]
pub fn fully_centered_content(
    content_width: u16,
    area_width: u16,
    area: Rect,
) -> Rect {
    let height = (content_width.saturating_div(area_width)).saturating_add(1);

    grid_center(
        Constraint::Length(area_width),
        Constraint::Length(height),
        area,
    )
}
