use std::collections::HashMap;

use mistserver_rs::{MistClientBuilder, StreamBuilder};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mut streams_hashmap = HashMap::new();

    let mut mist_client = MistClientBuilder::new("http://localhost:4242/api")
        .with_client(client)
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

    let stream = StreamBuilder::new("push://").name("stream_one").build();
    streams_hashmap.insert("stream_one".into(), stream);

    match mist_client.streams().create(streams_hashmap).await {
        Ok(r) => {
            println!("insertion successful, {r:?}")
        }
        Err(e) => {
            println!("insertion failed, e: {e:?}");
        }
    }
}
