//! Interface to store the useful data of a room (messages, name, handle, etc.)
//! to interface it simply.

use std::sync::Arc;

use matrix_sdk::room::MessagesOptions;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::{OwnedRoomId, UInt};
use matrix_sdk::{Room, RoomState, StoreError};
use serde::{Deserialize, Serialize};

/// Interface to display a room
///
/// If one of the fields failed to load, the field will contain an error.
pub struct DisplayRoom {
    /// Matrix room
    messages: Result<Vec<Message>, matrix_sdk::Error>,
    /// Room's list of messages
    name:     Result<String, StoreError>,
    /// Inner associated matrix room
    room:     Room,
    /// Room unique identifier
    room_id:  OwnedRoomId,
}

impl DisplayRoom {
    /// List messages of room
    ///
    /// # Errors
    ///
    /// Returns an error if the user failed to fetch the messages on the server.
    pub fn as_messages(&self) -> Result<&[Message], &matrix_sdk::Error> {
        self.messages.as_ref().map(Vec::as_slice)
    }

    /// Returns the room's name
    ///
    /// # Errors
    ///
    /// Returns an error if the [algorithm][1] to compute the room name fails.
    ///
    /// [1]: <https://matrix.org/docs/spec/client_server/latest#calculating-the-display-name-for-a-room>
    pub const fn as_name(&self) -> Result<&String, &StoreError> {
        self.name.as_ref()
    }

    /// Indicates whether an invitation is pending for this room.
    #[must_use]
    pub fn has_invitation(&self) -> bool {
        matches!(self.room.state(), RoomState::Invited)
    }

    /// Returns the room id
    #[must_use]
    pub const fn id(&self) -> &OwnedRoomId {
        &self.room_id
    }

    /// Clones and returns the inner room
    #[must_use]
    pub fn into_room(&self) -> RoomWrap {
        RoomWrap(self.room.clone())
    }

    /// Create a new display room from a [`Room`]
    pub async fn new(room: Room) -> Self {
        let name = room.display_name().await.map(|name| name.to_string());
        let mut opts = MessagesOptions::forward();
        opts.limit = UInt::MAX;
        let messages = room.messages(opts).await.map(|messages| {
            messages
                .chunk
                .into_iter()
                .filter_map(|msg| {
                    let message = msg.into_raw();
                    if let Ok(Some(msg_type)) =
                        message.get_field::<String>("type")
                        && msg_type == "m.room.message"
                        && let Ok(Some(body)) = message.get_field("content")
                    {
                        Some(body)
                    } else {
                        None
                    }
                })
                .collect()
        });
        let room_id = room.room_id().to_owned();
        Self { messages, name, room, room_id }
    }

    /// Updates the content of a room but the contents of another room
    pub fn update_from(&mut self, other: Self) {
        self.messages = other.messages;
        self.name = other.name;
    }
}

#[derive(Serialize, Deserialize)]
/// Content to extract from a message to display it
pub struct Message {
    /// Content of a message
    body: String,
}

impl Message {
    /// Returns the body of the message
    #[must_use]
    pub const fn as_body(&self) -> &str {
        self.body.as_str()
    }
}

/// Room wrapper to only keep the room wrapper.
pub struct RoomWrap(Room);

impl RoomWrap {
    /// Accepts the invitation received to join the room.
    ///
    /// # Errors
    ///
    /// If the room isn't in the "Invited" or "Left" state, or for regular
    /// connection errors.
    pub async fn accept_invitation(&self) -> Result<(), matrix_sdk::Error> {
        self.0.join().await
    }

    /// Sends a message in a room
    ///
    /// # Errors
    ///
    /// Returns an error when join handle crashes.
    pub async fn send_plain(&self, msg: &str) -> Result<(), matrix_sdk::Error> {
        self.0.send(RoomMessageEventContent::text_plain(msg)).await?;
        Ok(())
    }
}
