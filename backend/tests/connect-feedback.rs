use core::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;

use backend::user::User;
use matrix_sdk::Client;
use matrix_sdk::ruma::events::room::message::{
    RoomMessageEventContent, SyncRoomMessageEvent
};
use matrix_sdk_test::{JoinedRoomBuilder, SyncResponseBuilder};

#[tokio::test]
async fn test() {
    tracing_subscriber::fmt::init();

    println!("creating client");
    let client = Client::builder()
        .homeserver_url("http://example.org")
        .build()
        .await
        .unwrap();

    println!("creating user");
    let user = User::from(client.clone());

    println!("creating writer");
    let last_message = Arc::new(Mutex::new(String::new()));
    let last_message_writer = last_message.clone();
    let _ = user.on_receive_message(move |ev: SyncRoomMessageEvent| {
        let mut writer = last_message_writer.lock().unwrap();
        *writer = ev.as_original().unwrap().content.body().to_string();
    });

    println!("creating room");
    let room = JoinedRoomBuilder::default();

    println!("joining room");
    let sync_response = SyncResponseBuilder::default()
        .add_joined_room(room)
        .build_sync_response();


    println!("creating synchronisation");
    let synchronisation_handler = user.enable_sync();

    println!("waiting for room room");
    let room = user.wait_until_visible_room();
    room.send(RoomMessageEventContent::text_plain("Hello from Rust"))
        .await
        .unwrap_or_else(|err| panic!("Failed to say hello: {err}"));

    thread::sleep(Duration::from_secs(2));
    assert!(last_message.lock().unwrap().as_str() == "Hello from Rust");

    synchronisation_handler.await.unwrap().unwrap();
}
