use matrix_sdk::{Client, config::SyncSettings, ruma::events::room::message::SyncRoomMessageEvent};

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .homeserver_url("http://localhost:8008")
        .build()
        .await
        .unwrap_or_else(|err| panic!("Failed to build client: {err:?}"));

    println!("Built client");

    client
        .matrix_auth()
        .login_username("@c:localhost", "c")
        .send()
        .await
        .unwrap_or_else(|err| panic!("Failed to login: {err:?}"));

    println!("User connected!");

    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!(
            "[{:?}] New message from {}: {}",
            ev.origin_server_ts(),
            ev.sender(),
            ev.as_original().unwrap().content.body(),
        );
    });

    client
        .sync(SyncSettings::default())
        .await
        .unwrap_or_else(|err| panic!("Failed to synchronise with the server: {err:?}"));

    unreachable!("Sync function never returns")
}
