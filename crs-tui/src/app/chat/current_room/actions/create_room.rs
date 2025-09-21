use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::prompt::Prompt;

/// Component to create a room, with a given name
pub struct CreateRoom(Prompt<String>);

impl CreateRoom {
    /// Create a new [`CreateRoom`] with the right titles.
    pub const fn new() -> Self {
        Self(Prompt::new(
            Input::new().with_active(),
            " Name of the room to create ",
        ))
    }
}

impl Component for CreateRoom {
    type ResponseData = <Prompt<String> as Component>::ResponseData;
    type UpdateState = <Prompt<String> as Component>::UpdateState;

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        self.0.on_event(event).await
    }

    fn update(&mut self, response_data: Self::ResponseData) {
        self.0.update(response_data);
    }

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        self.0.draw(frame, area);
    }
}

/// Action to request a room creation
pub struct CreateRoomAction(pub String);
