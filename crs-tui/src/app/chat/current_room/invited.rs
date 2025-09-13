//! Invitation popup to a channel

use core::convert::Infallible;

use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::ui::component::Component;
use crate::ui::widgets::{InstructionsBuilder, linear_center};

/// Represents the action of accepting an invitation
pub struct AcceptInvitation;

/// Invitation popup to a channel
pub struct InvitationPopup;

impl Component for InvitationPopup {
    type ResponseData = Infallible;
    type UpdateState = AcceptInvitation;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let center =
            linear_center(Constraint::Length(4), Direction::Vertical, area);

        let layout = Layout::new(Direction::Vertical, [
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(center);

        let message =
            Paragraph::new("You have a pending invitation for this room.")
                .centered();
        frame.render_widget(message, layout[0]);

        let accept = InstructionsBuilder::default()
            .text("Press")
            .key("Enter")
            .text("to accept and join.")
            .build();

        let button_rect = linear_center(
            Constraint::Length(accept.width.saturating_add(2)),
            Direction::Horizontal,
            layout[1],
        );

        let button = Paragraph::new(accept.line)
            .centered()
            .block(Block::new().borders(Borders::ALL));

        frame.render_widget(button, button_rect);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        if let Some(key_event) = event.as_key_press_event()
            && key_event.code.is_enter()
        {
            Some(AcceptInvitation)
        } else {
            None
        }
    }
}
