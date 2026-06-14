use mistserver_rs::MistClientBuilder;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mist_client = MistClientBuilder::new("http://localhost:4242/api")
        .with_client(client)
        .build();

    let is_auth_enabled = mist_client.auth_enabled();
    println!("Auth status: {is_auth_enabled}")
}
