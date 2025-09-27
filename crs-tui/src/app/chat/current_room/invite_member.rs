//! Component to invite members to a room

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;
use std::sync::Mutex;

use crs_backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::widgets::grid_center;
use crate::utils::safe_unlock;

/// Popup widget to invite a member to the current room
pub struct InviteMemberPopup {
    /// Invitiation error, if exists
    error:  Option<String>,
    /// Name of the person to add to the current room
    person: Input<'static>,
    /// Room to which people need to be added
    room:   Arc<Mutex<DisplayRoom>>,
}

impl InviteMemberPopup {
    /// Returns the underlying room of an invitation popup
    pub fn into_room(self) -> Arc<Mutex<DisplayRoom>> {
        self.room
    }

    /// Create the [`InviteMemberPopup`] component
    pub const fn new(room: Arc<Mutex<DisplayRoom>>) -> Self {
        Self {
            error: None,
            person: Input::new()
                .with_active()
                .with_label("Name of the person to add"),
            room,
        }
    }
}

impl Component for InviteMemberPopup {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    #[expect(clippy::arithmetic_side_effects, reason = "width >= 50")]
    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let height = Input::HEIGHT_WITH_LABEL + 2;
        let width = (area.width - 2).min(50);

        let center = grid_center(
            Constraint::Length(width),
            Constraint::Length(height),
            area,
        );

        let layout = Layout::new(Direction::Vertical, [
            Constraint::Length(Input::HEIGHT_WITH_LABEL),
            Constraint::Length(2),
        ])
        .split(center);

        self.person.draw(frame, layout[0]);

        if let Some(err) = &self.error {
            frame.render_widget(
                Text::from(err.as_str()).style(Style::new().fg(Color::Red)),
                layout[1],
            );
        }
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        if let Some(key_event) = event.as_key_press_event()
            && key_event.code.is_enter()
        {
            let room = safe_unlock(&self.room).as_room();
            if let Err(err) = room.invite_user(&self.person.take_value()).await
            {
                self.error = Some(err.to_string());
            }
        }

        self.person.on_event(event).await
    }
}
