use matrix_sdk::Client;

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .homeserver_url("http://localhost:8008")
        .build()
        .await
        .unwrap_or_else(|err| panic!("Failed to build client: {err:?}"));

    client
        .matrix_auth()
        .login_username("@b:localhost", "b")
        .send()
        .await
        .unwrap_or_else(|err| panic!("Failed to login: {err:?}"));

    println!("Successfully logged in");
    dbg!(client);
}
