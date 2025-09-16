//! Data structure to contain the data of a logged in user

use core::convert::Infallible;
use core::time::Duration;
use std::thread;

use matrix_sdk::config::SyncSettings;
use matrix_sdk::event_handler::EventHandlerHandle;
use matrix_sdk::ruma::api::client::room::create_room::v3::Request as CreateRoomRequest;
use matrix_sdk::ruma::events::room::message::SyncRoomMessageEvent;
use matrix_sdk::{Client, ClientBuildError, Error, Room};
use tokio::task::{JoinError, JoinHandle};

use crate::room::DisplayRoom;

/// Connected user to the homeserver
pub struct User {
    /// Client to communicate with the homeserver
    client:   Client,
    /// Homeserver username
    username: Option<String>,
}

impl User {
    /// Create a new room and invite a user to this room
    ///
    /// # Errors
    ///
    /// - When the room creation failed
    /// - When the invination failed
    pub async fn create_room_with_name(
        &self,
        name: Option<String>,
    ) -> Result<Room, Error> {
        let mut req = CreateRoomRequest::new();
        req.name = name;
        self.client.create_room(req).await
    }

    /// Enable synchronisation with homeserver
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut user = User::new("http://localhost:8000").await.unwrap();
    /// user.login("@b:localhost", "b").await.unwrap();
    /// let sync_handle = user.enable_sync();
    ///
    /// // use user, it will be synced with the server automatically
    ///
    /// sync_handle.await.unwrap().unwrap();
    /// unreachable!()
    /// ```
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

    /// List all the rooms visible by the users
    ///
    /// A room is visible if the user joined, was invited or left the room.
    #[must_use]
    pub async fn list_rooms(&self) -> Vec<DisplayRoom> {
        let mut rooms: Vec<DisplayRoom> = vec![];
        for room in self.client.rooms() {
            rooms.push(DisplayRoom::new(room).await);
        }
        rooms
    }

    /// Loads the rooms concurrently and give the room when ready
    ///
    /// # Errors
    ///
    /// Returns an error when one of the tokio tasks paniced.
    ///
    /// This doesn't return an error if the room failed to load, the room will
    /// cotnain results. Refer to [`DisplayRoom`] for more information.
    pub async fn load_rooms<OnRoomLoad>(
        &self,
        on_room_load: OnRoomLoad,
    ) -> Result<(), JoinError>
    where
        OnRoomLoad: Fn(DisplayRoom) + Clone + Send + Sync + 'static,
    {
        let rooms = self.client.rooms();

        let mut futures = vec![];
        for room in rooms {
            let callback = on_room_load.clone();
            let handle = tokio::spawn(async move {
                callback(DisplayRoom::new(room).await);
            });
            futures.push(handle);
        }

        for handle in futures {
            handle.await?;
        }

        Ok(())
    }

    /// Log the user in with credentials
    ///
    /// # Errors
    ///
    /// Returns an error if the client failed to log in the homeserver.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut user = User::new("http://localhost:8000").await.unwrap();
    /// user.login("@b:localhost", "b").await.unwrap();
    /// ```
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
    ///
    /// This function will never panic or return an error, it will just run
    /// indefinetly until a room is visible from the [`User`].
    pub fn wait_for_visible_room(&self) {
        let mut backoff = 1;
        loop {
            thread::sleep(Duration::from_secs(backoff));
            backoff <<= 1_i32;
            if self.client.rooms().into_iter().next().is_some() {
                return;
            }
        }
    }
}
