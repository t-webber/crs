use matrix_sdk::{Room, StoreError};

/// Interface to display a room
pub struct DisplayRoom {
    /// Room's name
    name: Result<String, StoreError>,
    /// Matrix room
    room: Room,
}

impl DisplayRoom {
    /// Returns the room's name
    pub const fn as_name(&self) -> &Result<String, StoreError> {
        &self.name
    }

    /// Create a new display room from a [`Room`]
    pub async fn new(room: Room) -> Self {
        let name = room.display_name().await.map(|name| name.to_string());
        Self { name, room }
    }
}
