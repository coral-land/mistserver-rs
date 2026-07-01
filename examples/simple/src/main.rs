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

    let builder_result = StreamBuilder::new(
        "stream_some",
        "https://st1101.gapfilm.ir/s/2026/1/6971f6b03fb44d894fd9ea70/c_x264_1280.mp4/chunk.m3u8?mk=tv8lwyGF_egMa2LZctI1Ww&si=786c6d55-3074-4297-b673-9d145e361da7&sc=GF_WEBSITE&app=Web&ts=Gapfilm",
    )
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
}
