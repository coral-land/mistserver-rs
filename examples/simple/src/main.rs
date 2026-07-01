use std::{thread::sleep, time::Duration};

use mistserver_rs::{MistClientBuilder, StreamBuilder};
use reqwest::Client;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .finish();

    let client = Client::new();

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

    let builder_result = StreamBuilder::new("stream_some", "/video/file.mp4")
        .always_on(true)
        .debug(10)
        .build();

    match builder_result {
        Err(e) => println!("Error in building stream: {e}"),
        Ok(stream) => match mist_client.streams().create_one(stream).await {
            Ok(r) => {
                println!("insertion successful, {r:?}")
            }
            Err(e) => {
                println!("insertion failed, e: {e:?}");
            }
        },
    }

    // TODO: sleep
    println!("Sleeping for 5 seconds ...");
    sleep(Duration::from_secs(5));

    // TODO: Update
    // TODO: List
}
