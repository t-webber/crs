use matrix_sdk::room::MessagesOptions;
use matrix_sdk::ruma::UInt;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::{Room, StoreError};
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

    /// Create a new display room from a [`Room`]
    pub async fn new(room: Room) -> Self {
        let name = room.display_name().await.map(|name| name.to_string());
        let mut opts = MessagesOptions::backward();
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
