extern crate alloc;
use alloc::sync::Arc;
use core::fmt::{self, Display, Formatter};
use std::sync::Mutex;

use crs_backend::room::DisplayRoom;

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
    pub fn as_room(&self) -> Arc<Mutex<DisplayRoom>> {
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
pub struct RoomSearch(Prompt<NamedRoom>);

impl RoomSearch {
    fn new(room_list: Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>) -> Self {
        let named_room_list = safe_unlock(&room_list)
            .iter()
            .filter_map(|room| Arc::clone(&room).try_into().ok())
            .collect();
        Self(Prompt::new_with_list(
            Input::new().with_active(),
            " Name fo the room ",
            named_room_list,
        ))
    }
}
