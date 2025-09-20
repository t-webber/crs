//! Interface to store the useful data of a room (messages, name, handle, etc.)
//! to interface it simply.

extern crate alloc;
use alloc::sync::Arc;
use core::fmt::{self, Display, Formatter};

use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::ruma::{OwnedRoomId, UserId};
use matrix_sdk::{Room, RoomState, StoreError};

use crate::message::{DisplayMessage, get_room_messages};

/// Interface to display a room
///
/// If one of the fields failed to load, the field will contain an error.
pub struct DisplayRoom {
    /// Matrix room
    messages: Result<Vec<DisplayMessage>, matrix_sdk::Error>,
    /// Room's list of messages
    name:     Result<Arc<str>, StoreError>,
    /// Inner associated matrix room
    room:     Arc<Room>,
    /// Room unique identifier
    room_id:  OwnedRoomId,
}

impl DisplayRoom {
    /// List messages of room
    ///
    /// # Errors
    ///
    /// Returns an error if the user failed to fetch the messages on the server.
    pub fn as_messages(&self) -> Result<&[DisplayMessage], &matrix_sdk::Error> {
        self.messages.as_ref().map(Vec::as_slice)
    }

    /// Returns the room's name
    ///
    /// # Errors
    ///
    /// Returns an error if the [algorithm][1] to compute the room name fails.
    ///
    /// [1]: <https://matrix.org/docs/spec/client_server/latest#calculating-the-display-name-for-a-room>
    pub fn as_name(&self) -> Result<Arc<str>, &StoreError> {
        self.name.as_ref().cloned()
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

    /// Create a new room and invite a user to this room
    ///
    /// # Errors
    ///
    /// - When the room creation failed
    /// - When the invination failed
    pub async fn invite_user(
        &self,
        user_id: &str,
    ) -> Result<(), matrix_sdk::Error> {
        self.room.invite_user_by_id(&UserId::parse(user_id)?).await
    }

    /// Create a new display room from a [`Room`]
    pub async fn new(room: Arc<Room>) -> Self {
        let name = get_room_name(&room).await;
        let messages = get_room_messages(&room).await;

        let room_id = room.room_id().to_owned();
        Self { messages, name, room, room_id }
    }

    /// Refreshes the name and the messages of a room
    pub async fn refresh(&mut self) {
        self.update_with(
            get_room_messages(&self.room).await,
            get_room_name(&self.room).await,
        );
    }

    /// Updates the content of a room but the contents of another room
    pub fn update_from(&mut self, other: Self) {
        self.update_with(other.messages, other.name);
    }

    /// Updates the content of a room with another name and some other messages.
    fn update_with(
        &mut self,
        messages: Result<Vec<DisplayMessage>, matrix_sdk::Error>,
        name: Result<Arc<str>, StoreError>,
    ) {
        if messages.is_ok() || self.messages.is_err() {
            self.messages = messages;
        }
        if name.is_ok() || self.name.is_err() {
            self.name = name;
        }
    }
}

/// Room that has for sure a name
pub struct NamedRoom {
    /// Name of the room
    name: Arc<str>,
    /// Underlying room
    room: Arc<Room>,
}

impl NamedRoom {
    /// Returns the name of the room
    #[must_use]
    pub fn as_name(&self) -> Arc<str> {
        Arc::clone(&self.name)
    }

    /// Return the underlying room object to do some actions on the matrix room
    #[must_use]
    pub fn as_room(&self) -> RoomWrap {
        RoomWrap(Arc::clone(&self.room))
    }
}

impl Display for NamedRoom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.as_name(), f)
    }
}

impl<'room> TryFrom<&'room DisplayRoom> for NamedRoom {
    type Error = &'room StoreError;

    fn try_from(value: &'room DisplayRoom) -> Result<Self, Self::Error> {
        Ok(Self {
            name: Arc::clone(&value.as_name()?),
            room: Arc::clone(&value.room),
        })
    }
}

/// Room wrapper to only keep the room wrapper.
pub struct RoomWrap(Arc<Room>);

impl RoomWrap {
    /// Accepts the invitation received to join the room.
    ///
    /// The data is then refetched.
    ///
    /// # Errors
    ///
    /// If the room isn't in the "Invited" or "Left" state, or for regular
    /// connection errors.
    pub async fn accept_invitation(
        self,
    ) -> Result<DisplayRoom, matrix_sdk::Error> {
        self.0.join().await?;
        Ok(DisplayRoom::new(self.0).await)
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

/// Computes the name of a room
///
/// # Errors
///
/// Returns an error if the [algorithm][1] to compute the room name fails.
///
/// [1]: <https://matrix.org/docs/spec/client_server/latest#calculating-the-display-name-for-a-room>
pub async fn get_room_name(room: &Room) -> Result<Arc<str>, StoreError> {
    room.display_name().await.map(|name| Arc::from(name.to_room_alias_name()))
}
