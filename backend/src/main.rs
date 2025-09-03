use std::{thread, time::Duration};

use matrix_sdk::{
    Client,
    config::SyncSettings,
    ruma::{
        TransactionId,
        events::room::message::{RoomMessageEventContent, SyncRoomMessageEvent},
    },
};

async fn sync_client(client: Client) {
    client
        .sync(SyncSettings::default())
        .await
        .unwrap_or_else(|err| panic!("Failed to synchronise with the server: {err:?}"));
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
                .with_transaction_id(TransactionId::new())
                .await
                .unwrap();
            println!("\x1b[35mMessage sent!\x1b[0m");
            break;
        }
    }

    synchronisation_handler.await.unwrap();
    unreachable!("Sync function never returns")
}
