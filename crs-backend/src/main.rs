#![feature(impl_trait_in_bindings)]

use matrix_sdk::ruma::events::room::message::{
    RoomMessageEventContent, SyncRoomMessageEvent
};
use matrix_sdk::ruma::user_id;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut user = backend::user::User::new("http://localhost:8008")
        .await
        .unwrap_or_else(|err| panic!("Failed to build client: {err:?}"));

    user.login("@c:localhost".into(), "c")
        .await
        .unwrap_or_else(|err| panic!("Failed to login: {err:?}"));

    let _ = user.on_receive_message(|ev: SyncRoomMessageEvent| {
        println!(
            "\x1b[35m[{:?}] New message from {}: {}\x1b[0m",
            ev.origin_server_ts(),
            ev.sender(),
            ev.as_original().unwrap().content.body()
        )
    });

    let synchronisation_handler = user.enable_sync();

    let room = user.wait_until_visible_room().await;
    room.send_plain("Hello from Rust")
        .await
        .unwrap_or_else(|err| panic!("Failed to say hello: {err}"));

    let whatsapp_dm = user
        .create_room_with(user_id!("@whatsappbot:localhost"))
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to create DM with whatsappbot: {err}")
        });

    whatsapp_dm
        .send(RoomMessageEventContent::text_plain("!wa login qr"))
        .await
        .unwrap_or_else(|err| panic!("Failed to reach whatsapp: {err}"));

    synchronisation_handler.await.unwrap().unwrap();
}
