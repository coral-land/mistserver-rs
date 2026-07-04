//! Complete MistServer SDK example.
//!
//! This example demonstrates the recommended workflow for interacting with a
//! MistServer instance using the `mistserver-rs` SDK.
//!
//! It performs:
//! 1. Creates a reusable HTTP client.
//! 2. Builds and authenticates a `MistClient`.
//! 3. Creates a single stream (`demo_stream`).
//! 4. Creates multiple streams in a batch (`batch_stream_1`, `2`, `3`).
//! 5. Lists all streams and active streams.
//! 6. Adds tags to `demo_stream` and lists again.
//! 7. Removes tags from `demo_stream`.
//! 8. Nukes `demo_stream` and deletes the batch streams.

use anyhow::{Context, Result};
use mistserver_rs::{MistClient, MistClientBuilder, StreamBuilder};
use reqwest::Client;
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;
use tracing::{Level, info, warn};

const MIST_ENDPOINT: &str = "http://localhost:4242/api2";
const MIST_USERNAME: &str = "admin";
const MIST_PASSWORD: &str = "password";

const STREAM_SOURCE_1: &str = "/video/file.mp4";
const STREAM_SOURCE_2: &str = "/video/file.mp4";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .compact()
        .with_line_number(false)
        .with_thread_ids(false)
        .with_target(false)
        .with_level(true)
        .init();

    info!("Starting MistServer SDK example");

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .context("Failed to create HTTP client")?;

    let mut mist_client = MistClientBuilder::new(MIST_ENDPOINT)
        .with_client(client)
        .with_auth(MIST_USERNAME, MIST_PASSWORD)
        .build();

    mist_client
        .auth()
        .authorize()
        .await
        .context("Authentication failed")?;

    info!("Authenticated successfully");

    // Create a dedicated stream for tag operations
    let demo_stream_name = "demo_stream";
    create_demo_stream(&mut mist_client, demo_stream_name).await?;

    // Create batch streams
    let batch_names = create_batch_streams(&mut mist_client).await?;

    // Tag the demo stream
    info!(
        stream = demo_stream_name,
        "Adding tags 'demo' and 'example'"
    );
    mist_client
        .streams()
        .tag(demo_stream_name, vec!["demo", "example"])
        .await
        .context("Failed to add tags")?;

    // List active streams to see tags (if returned)
    let active = mist_client.streams().list_active().await?;
    info!(active_streams = ?active, "Active streams after tagging");

    // Remove one tag from the demo stream
    info!(stream = demo_stream_name, "Removing tag 'demo'");
    mist_client
        .streams()
        .untag(demo_stream_name, vec!["demo"])
        .await
        .context("Failed to remove tag")?;

    let list_response = mist_client.streams().list().await?;
    info!(
        total_streams = list_response.streams.len(),
        streams = ?list_response.streams,
        "Full stream list after untagging"
    );

    info!("Waiting 5 seconds for demonstration…");
    sleep(Duration::from_secs(5)).await;

    // Cleanup: nuke the demo stream, delete batch streams
    info!(stream = demo_stream_name, "Nuking demo stream");
    mist_client
        .streams()
        .nuke_stream(demo_stream_name.to_string())
        .await
        .context("Failed to nuke demo stream")?;

    if !batch_names.is_empty() {
        info!(streams = ?batch_names, "Deleting batch streams");
        mist_client
            .streams()
            .delete(batch_names)
            .await
            .context("Failed to delete batch streams")?;
    }

    info!("Example completed successfully");
    Ok(())
}

async fn create_demo_stream(mist_client: &mut MistClient, name: &str) -> Result<()> {
    info!(stream = name, "Creating demo stream");

    let stream = StreamBuilder::new(name, STREAM_SOURCE_1)
        .always_on(true)
        .debug(10)
        .build()
        .context("Failed to build demo stream config")?;

    mist_client
        .streams()
        .add(stream)
        .await
        .context("Failed to create demo stream")?;

    info!(stream = name, "Demo stream created");
    Ok(())
}

async fn create_batch_streams(mist_client: &mut MistClient) -> Result<Vec<String>> {
    info!("Preparing batch stream configurations");

    let configs = vec![
        ("batch_stream_1", STREAM_SOURCE_1),
        ("batch_stream_2", STREAM_SOURCE_1),
        ("batch_stream_3", STREAM_SOURCE_2),
    ];

    let mut streams = HashMap::with_capacity(configs.len());
    let mut names = Vec::with_capacity(configs.len());

    for (name, source) in configs {
        match StreamBuilder::new(name, source)
            .always_on(true)
            .debug(5)
            .build()
        {
            Ok(stream) => {
                streams.insert(name.to_string(), stream);
                names.push(name.to_string());
            }
            Err(e) => warn!(stream = name, error = %e, "Skipping invalid config"),
        }
    }

    if streams.is_empty() {
        anyhow::bail!("No valid stream configurations");
    }

    mist_client
        .streams()
        .add_many(streams)
        .await
        .context("Batch creation failed")?;

    info!("Successfully created {} batch streams", names.len());
    Ok(names)
}
