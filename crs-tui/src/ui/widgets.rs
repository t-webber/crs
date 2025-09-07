//! Module that contains reusable widgets and UI elements for different screens
//! and components

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Stylize as _;
use ratatui::text::Line;

/// Widget to display instructions on the screen
pub struct Instructions<'text> {
    /// widget line
    pub line:  Line<'text>,
    /// space taken by the widget, for usage in constraints, layouts, etc.
    pub width: u16,
}

impl<'text> Instructions<'text> {
    /// Formats a list of instructions to display on the screen for keys to
    /// press.
    ///
    /// # Arguments
    ///
    /// Pass in input a list of couples:
    /// - the first element is the action description
    /// - the second element is the key to press
    ///
    /// Keep it short or it won't fit on the screen!
    pub fn new(instructions: &[(&'text str, &'text str)]) -> Self {
        let mut width: u16 = 1;
        let mut line_elements = vec![" ".into()];
        for &(action, key) in instructions {
            width = width
                .saturating_add(saturating_cast(action.len()))
                .saturating_add(saturating_cast(action.len()))
                .saturating_add(2);

            line_elements.push(action.into());
            line_elements.push(format!(" <{key}> ").yellow().bold());
        }
        Self { line: Line::from(line_elements), width }
    }
}

/// Cast a [`usize`] to [`u16`] with saturation
///
/// If the input is too large to fit in a [`u16`]
/// [`u16::MAX`] is used
fn saturating_cast(value: usize) -> u16 {
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
    #[expect(clippy::indexing_slicing, reason = "len = 3")]
    Layout::default()
        .direction(direction)
        .constraints([
            Constraint::Fill(1),
            middle_constraint,
            Constraint::Fill(1),
        ])
        .split(area)[1]
}
