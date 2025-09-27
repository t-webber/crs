//! UI component to search and open a room by typing its name

extern crate alloc;
use alloc::sync::Arc;
use core::fmt::{self, Display, Formatter};
use std::sync::Mutex;

use crs_backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::prompt::Prompt;
use crate::utils::safe_unlock;

/// Room that has for sure a name
pub struct NamedRoom {
    /// Name of the room
    name: Arc<str>,
    /// Underlying room
    room: Arc<Mutex<DisplayRoom>>,
}

impl NamedRoom {
    /// Returns the name of the room
    #[must_use]
    pub fn as_name(&self) -> Arc<str> {
        Arc::clone(&self.name)
    }

    /// Return the underlying room object to do some actions on the matrix room
    #[must_use]
    fn as_room(&self) -> Arc<Mutex<DisplayRoom>> {
        Arc::clone(&self.room)
    }
}

impl Display for NamedRoom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.as_name(), f)
    }
}

impl TryFrom<Arc<Mutex<DisplayRoom>>> for NamedRoom {
    type Error = ();

    fn try_from(value: Arc<Mutex<DisplayRoom>>) -> Result<Self, Self::Error> {
        let name = safe_unlock(&value).as_name().ok_or(())?;
        let room = Arc::clone(&value);
        Ok(Self { name, room })
    }
}

/// Component to search a room by name
pub struct RoomSearch {
    /// Prompt UI component to search a room
    prompt: Prompt<NamedRoom>,
    /// List of loaded rooms to find from
    rooms:  Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>,
}

impl RoomSearch {
    /// Creates a new [`RoomSearch`] component.
    ///
    /// Only the rooms with a valid name can be chosen
    pub fn new(rooms: Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>) -> Self {
        let named_rooms = safe_unlock(&rooms)
            .iter()
            .filter_map(|room| Arc::clone(room).try_into().ok())
            .collect();
        Self {
            prompt: Prompt::new_with_list(
                Input::new().with_active(),
                " Name fo the room ",
                named_rooms,
            ),
            rooms,
        }
    }
}

impl Component for RoomSearch {
    type ResponseData = <Prompt<NamedRoom> as Component>::ResponseData;
    type UpdateState = Arc<Mutex<DisplayRoom>>;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        self.prompt.draw(frame, area);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let name = self.prompt.on_event(event).await?;
        safe_unlock(&self.rooms)
            .iter()
            .find(|room| {
                safe_unlock(room)
                    .as_name()
                    .is_some_and(|room_name| room_name.as_ref() == name)
            })
            .cloned()
    }

    fn update(&mut self, response_data: Self::ResponseData) {
        self.prompt.update(response_data);
    }
}
