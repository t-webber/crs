//! Data structure to contain the data of a logged in user

use core::convert::Infallible;
use core::time::Duration;
use std::thread;

use matrix_sdk::config::SyncSettings;
use matrix_sdk::event_handler::EventHandlerHandle;
use matrix_sdk::ruma::UserId;
use matrix_sdk::ruma::api::client::room::create_room::v3::Request as CreateRoomRequest;
use matrix_sdk::ruma::events::room::message::SyncRoomMessageEvent;
use matrix_sdk::{Client, ClientBuildError, Error, Room};
use tokio::task::JoinHandle;

/// Connected user to the homeserver
pub struct User {
    /// Client to communicate with the homeserver
    pub client: Client,
    /// Homeserver username
    username:   Option<String>,
}

impl User {
    /// Create a new room and invite a user to this room
    ///
    /// # Errors
    ///
    /// - When the room creation failed
    /// - When the invination failed
    pub async fn create_room_with(
        &self,
        user_id: &UserId,
    ) -> Result<Room, Error> {
        let room = self.client.create_room(CreateRoomRequest::new()).await?;
        room.invite_user_by_id(user_id).await?;
        Ok(room)
    }

    /// Enable synchronisation with homeserver
    #[must_use]
    pub fn enable_sync(&self) -> JoinHandle<Result<Infallible, Error>> {
        tokio::spawn({
            let client = self.client.clone();
            async move {
                client.sync(SyncSettings::default()).await?;
                Err(Error::UnknownError(
                    "Synchronisation was terminated".into(),
                ))
            }
        })
    }

    /// Log the user in with credentials
    ///
    /// # Errors
    ///
    /// Returns an error if the client failed to log in the homeserver.
    pub async fn login(
        &mut self,
        username: String,
        password: &str,
    ) -> Result<(), Error> {
        self.client
            .matrix_auth()
            .login_username(&username, password)
            .send()
            .await?;
        self.username = Some(username);
        Ok(())
    }

    /// Create a new client to the homeserver
    ///
    /// # Errors
    ///
    /// Returns a [`ClientBuildError`] if the client failed to connect to the
    /// homeserver.
    pub async fn new(url: &str) -> Result<Self, ClientBuildError> {
        let client = Client::builder().homeserver_url(url).build().await?;
        Ok(Self { username: None, client })
    }

    /// Calls a handler when an incomming message is received
    #[must_use]
    pub fn on_receive_message<F>(&self, handler: F) -> EventHandlerHandle
    where F: Fn(SyncRoomMessageEvent) + Clone + Sync + Send + 'static {
        self.client.add_event_handler(
            async move |event: SyncRoomMessageEvent| handler(event),
        )
    }

    /// Wait until the client can see a room.
    ///
    /// A room is visible if the user joined, was invited or left the room.
    #[must_use]
    pub fn wait_until_visible_room(&self) -> Room {
        let mut backoff = 1;
        loop {
            thread::sleep(Duration::from_secs(backoff));
            backoff <<= 1;
            if let Some(room) = self.client.rooms().into_iter().next() {
                return room;
            }
        }
    }
}

impl From<Client> for User {
    fn from(client: Client) -> Self {
        Self { client, username: None }
    }
}
