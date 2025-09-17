//! Component that handles room creation

use core::convert::Infallible;

use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Rect};

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::widgets::grid_center;

/// Popup to create a room
pub struct CreateRoom {
    /// Input to enter the name of the room to be create
    name: Input<'static>,
}

impl CreateRoom {
    /// Create [`CreateRoom`] component
    pub const fn new() -> Self {
        Self {
            name: Input::new().with_active().with_label("Create a new room"),
        }
    }
}

impl Component for CreateRoom {
    type ResponseData = Infallible;
    type UpdateState = CreateRoomAction;

    #[expect(clippy::arithmetic_side_effects, reason = "width >= 20")]
    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let height = Input::HEIGHT_WITH_LABEL + 2;
        let width = (area.width - 2).min(50);

        let popup_area = grid_center(
            Constraint::Length(width),
            Constraint::Length(height),
            area,
        );

        self.name.draw(frame, popup_area);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        if let Some(key_event) = event.as_key_press_event()
            && key_event.code.is_enter()
        {
            return Some(CreateRoomAction(self.name.take_value()));
        }
        let _: Infallible = self.name.on_event(event).await?;
        None
    }
}

/// Action of validating the "create room" form.
///
/// This struct is the request sent to create a room.
///
/// The field contains the name of the room to be create
pub struct CreateRoomAction(pub String);
