//! UI component to create room with a given name

use crate::derive_component;
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
            vec![],
        ))
    }
}

derive_component!(CreateRoom, Prompt<String>);

/// Action to request a room creation
pub struct CreateRoomAction(pub String);
