use mistserver_rs::MistClientBuilder;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mist_client = MistClientBuilder::new("http://localhost:4242/api")
        .with_client(client)
        .with_auth("admin", "password")
        .build();
}
