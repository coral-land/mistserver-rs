use mistserver_rs::MistClientBuilder;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mut mist_client = MistClientBuilder::new("http://localhost:4242/api")
        .with_client(client)
        // Better to load these from env :)
        .with_auth("admin", "password")
        .build();

    match mist_client.authorize().await {
        Err(e) => {
            panic!("Error in authorization: {e:}")
        }
        Ok(_) => {
            println!("Authorization successful");
        }
    }
}
