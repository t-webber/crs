use std::{thread, time::Duration};

use matrix_sdk::{
    Client, RoomMemberships,
    config::SyncSettings,
    ruma::{
        api::client::room::create_room::v3::Request as CreateRoomRequest,
        events::room::message::{RoomMessageEventContent, SyncRoomMessageEvent},
        user_id,
    },
};

async fn sync_client(client: Client) {
    client
        .sync(SyncSettings::default())
        .await
        .unwrap_or_else(|err| panic!("Failed to synchronise with the server: {err:?}"));
}

async fn ask_for_whatsapp_login_qr_code(client: Client) {
    let whatsappbot_room = client.create_room(CreateRoomRequest::new()).await.unwrap();
    whatsappbot_room
        .invite_user_by_id(user_id!("@whatsappbot:localhost"))
        .await
        .unwrap();
    assert!(
        whatsappbot_room
            .members(RoomMemberships::ACTIVE)
            .await
            .unwrap()
            .into_iter()
            .any(|member| member.name() == "WhatsApp bridge Bot")
    );
    whatsappbot_room
        .send(RoomMessageEventContent::text_plain("!wa login qr"))
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = Client::builder()
        .homeserver_url("http://localhost:8008")
        .build()
        .await
        .unwrap_or_else(|err| panic!("Failed to build client: {err:?}"));

    client
        .matrix_auth()
        .login_username("@c:localhost", "c")
        .send()
        .await
        .unwrap_or_else(|err| panic!("Failed to login: {err:?}"));

    client.add_event_handler(async move |ev: SyncRoomMessageEvent| {
        println!(
            "\x1b[35m[{:?}] New message from {}: {}\x1b[0m",
            ev.origin_server_ts(),
            ev.sender(),
            ev.as_original().unwrap().content.body(),
        );
    });

    let synchronisation_handler = tokio::spawn(sync_client(client.clone()));

    let mut backoff = 1;
    loop {
        thread::sleep(Duration::from_secs(backoff));
        backoff <<= 1;
        if let Some(room) = client.rooms().first() {
            room.send(RoomMessageEventContent::text_plain("Hello from Rust"))
                .await
                .unwrap();
            println!("\x1b[35mMessage sent!\x1b[0m");
            break;
        }
    }

    ask_for_whatsapp_login_qr_code(client).await;

    synchronisation_handler.await.unwrap();
    unreachable!("Sync function never returns")
}
