use matrix_sdk::room::{Messages, MessagesOptions};
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::{Room, StoreError};

/// Interface to display a room
///
/// If one of the fields failed to load, the field will contain an error.
pub struct DisplayRoom {
    /// Matrix room
    messages: Result<Messages, matrix_sdk::Error>,
    /// Room's list of messages
    name:     Result<String, StoreError>,
    /// Inner associated matrix room
    room:     Room,
}

impl DisplayRoom {
    /// Returns the room's name
    pub const fn as_name(&self) -> &Result<String, StoreError> {
        &self.name
    }

    /// Create a new display room from a [`Room`]
    pub async fn new(room: Room) -> Self {
        let name = room.display_name().await.map(|name| name.to_string());
        let messages = room.messages(MessagesOptions::forward()).await;
        Self { messages, name, room }
    }

    /// Sends a message in a room
    ///
    /// # Errors
    ///
    /// Returns an error when join handle crashes.
    pub async fn send_plain(&self, msg: &str) -> Result<(), matrix_sdk::Error> {
        self.room.send(RoomMessageEventContent::text_plain(msg)).await?;
        Ok(())
    }
}
