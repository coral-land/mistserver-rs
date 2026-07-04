//! Complete MistServer SDK example.
//!
//! This example demonstrates the recommended workflow for interacting with a
//! MistServer instance using the `mistserver-rs` SDK.
//!
//! It performs the following steps:
//!
//! 1. Creates a reusable HTTP client.
//! 2. Builds a `MistClient`.
//! 3. Authenticates with the Mist API.
//! 4. Creates a single stream.
//! 5. Creates multiple streams in a single request.
//! 6. Lists all available streams.
//! 7. Cleans up **only** the streams created during this example.
//!
//! This example is intended to demonstrate the typical lifecycle of a
//! long-running application where a single `MistClient` instance is reused
//! for multiple API requests.

use anyhow::{Context, Result};
use mistserver_rs::{MistClient, MistClientBuilder, StreamBuilder};
use reqwest::Client;
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;
use tracing::{Level, info, warn};

// ---------------------------------------------------------------------
// Configuration constants
// ---------------------------------------------------------------------
const MIST_ENDPOINT: &str = "http://localhost:4242/api2";
const MIST_USERNAME: &str = "admin";
const MIST_PASSWORD: &str = "password";

const STREAM_SOURCE_1: &str = "/video/file.mp4";
const STREAM_SOURCE_2: &str = "/video/other.mp4";

#[tokio::main]
async fn main() -> Result<()> {
    // -----------------------------------------------------------------
    // Initialize logging
    // -----------------------------------------------------------------
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .compact()
        .with_line_number(false)
        .with_thread_ids(false)
        .with_target(false)
        .with_level(true)
        .init();

    info!("Starting MistServer SDK example");

    // -----------------------------------------------------------------
    // Build a reusable HTTP client
    // -----------------------------------------------------------------
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .context("Failed to create HTTP client")?;

    // -----------------------------------------------------------------
    // Create the Mist client
    // -----------------------------------------------------------------
    let mut mist_client = MistClientBuilder::new(MIST_ENDPOINT)
        .with_client(client)
        .with_auth(MIST_USERNAME, MIST_PASSWORD)
        .build();

    // -----------------------------------------------------------------
    // Authenticate once
    // -----------------------------------------------------------------
    mist_client
        .auth()
        .authorize()
        .await
        .context("Authentication with MistServer failed")?;

    info!("Successfully authenticated with MistServer");

    // -----------------------------------------------------------------
    // Create streams and collect their names for later cleanup
    // -----------------------------------------------------------------
    let mut created_streams = Vec::new();

    let single_name = add_single_stream(&mut mist_client).await?;
    created_streams.push(single_name);

    let multiple_names = add_multiple_streams(&mut mist_client).await?;
    created_streams.extend(multiple_names);

    // -----------------------------------------------------------------
    // List active streams (optional)
    // -----------------------------------------------------------------
    let active = mist_client.streams().list_active().await?;
    info!(active_streams = ?active, "Active streams after creation");

    // -----------------------------------------------------------------
    // List all streams
    // -----------------------------------------------------------------
    let list_response = mist_client.streams().list().await?;
    info!(
        total_streams = list_response.streams.len(),
        streams = ?list_response.streams,
        "Retrieved full stream list"
    );

    // -----------------------------------------------------------------
    // Wait for demonstration purposes (not required in production)
    // -----------------------------------------------------------------
    info!("Waiting 5 seconds for streams to initialise (demo only)…");
    sleep(Duration::from_secs(5)).await;

    // -----------------------------------------------------------------
    // Cleanup – delete only the streams we created
    // -----------------------------------------------------------------
    cleanup_streams(&mut mist_client, &created_streams).await?;

    info!("Example completed successfully");
    Ok(())
}

/// Creates a single stream and returns its name.
async fn add_single_stream(mist_client: &mut MistClient) -> Result<String> {
    let name = "stream_some";
    info!(stream = name, "Creating single stream");

    let stream = StreamBuilder::new(name, STREAM_SOURCE_1)
        .always_on(true)
        .debug(10)
        .build()
        .context("Failed to build stream configuration")?;

    mist_client
        .streams()
        .add(stream)
        .await
        .context("Failed to create stream")?;

    info!(stream = name, "Successfully created stream");
    Ok(name.to_string())
}

/// Creates multiple streams in a single batch request and returns their names.
async fn add_multiple_streams(mist_client: &mut MistClient) -> Result<Vec<String>> {
    info!("Preparing multiple stream configurations");

    let configs = vec![
        ("stream1", STREAM_SOURCE_1),
        ("stream2", STREAM_SOURCE_1),
        ("stream3", STREAM_SOURCE_2),
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
                info!(stream = name, "Prepared stream configuration");
                streams.insert(name.to_string(), stream);
                names.push(name.to_string());
            }
            Err(e) => {
                warn!(
                    stream = name,
                    error = %e,
                    "Skipping invalid stream configuration"
                );
            }
        }
    }

    if streams.is_empty() {
        anyhow::bail!("No valid stream configurations were created");
    }

    info!(
        total_streams = streams.len(),
        "Creating streams using batch request"
    );

    mist_client
        .streams()
        .add_many(streams)
        .await
        .context("Batch creation failed")?;

    info!("Successfully created {} streams", names.len());
    Ok(names)
}

/// Deletes the streams whose names are given.
async fn cleanup_streams(mist_client: &mut MistClient, names: &[String]) -> Result<()> {
    if names.is_empty() {
        info!("No streams to clean up");
        return Ok(());
    }

    info!(streams = ?names, "Cleaning up example streams");

    mist_client
        .streams()
        .delete(names.to_vec())
        .await
        .context("Failed to delete streams")?;

    info!("Cleanup completed for {} streams", names.len());
    Ok(())
}
