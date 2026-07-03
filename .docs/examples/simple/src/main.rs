//! Complete MistServer SDK example.
//!
//! This example demonstrates the recommended workflow for interacting with a
//! MistServer instance using the `mistserver-rs` SDK.
//!
//! The example performs the following steps:
//!
//! 1. Creates a reusable HTTP client.
//! 2. Builds a `MistClient`.
//! 3. Authenticates with the Mist API.
//! 4. Creates a single stream.
//! 5. Creates multiple streams in a single request.
//! 6. Lists all available streams.
//! 7. Cleans up by deleting the streams created during the example.
//!
//! This example is intended to demonstrate the typical lifecycle of a
//! long-running application where a single `MistClient` instance is reused
//! for multiple API requests.
//!
//! Expected output (simplified):
//!
//! INFO Successfully authenticated
//! INFO Created stream "stream_some"
//! INFO Created 3 streams
//! INFO Retrieved 4 streams
//! INFO Deleted 2 streams
//! INFO Example completed successfully

use mistserver_rs::{MistClient, MistClientBuilder, StreamBuilder};
use reqwest::Client;
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ---------------------------------------------------------------------
    // Initialize logging
    // ---------------------------------------------------------------------
    //
    // The example uses `tracing` for structured logging. Any logger
    // compatible with the `tracing` ecosystem may be used.
    //
    tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_level(true)
        .init();

    info!("Starting MistServer SDK example");

    // ---------------------------------------------------------------------
    // Build a reusable HTTP client
    // ---------------------------------------------------------------------
    //
    // Reusing a single reqwest client enables:
    //
    // - HTTP connection pooling
    // - Keep-alive support
    // - Shared timeout configuration
    //
    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    // ---------------------------------------------------------------------
    // Create the Mist client
    // ---------------------------------------------------------------------
    //
    // MistClientBuilder configures:
    //
    // - API endpoint
    // - Authentication credentials
    // - Custom HTTP client
    //
    // The resulting client should generally be reused throughout the
    // lifetime of your application.
    //
    let mut mist_client = MistClientBuilder::new("http://localhost:4242/api")
        .with_client(client)
        .with_auth("admin", "password")
        .build();

    // ---------------------------------------------------------------------
    // Authentication
    // ---------------------------------------------------------------------
    //
    // Authenticate once before issuing API requests.
    //
    // On success, the SDK stores the session internally and automatically
    // reuses it for subsequent requests.
    //
    mist_client
        .auth()
        .authorize()
        .await
        .map_err(|e| anyhow::anyhow!("Authentication failed: {e}"))?;

    info!("Successfully authenticated with MistServer");

    // ---------------------------------------------------------------------
    // Create streams
    // ---------------------------------------------------------------------

    add_single_stream(&mut mist_client).await?;
    add_multiple_streams(&mut mist_client).await?;

    // ---------------------------------------------------------------------
    // List streams
    // ---------------------------------------------------------------------
    //
    // Retrieve all currently configured streams.
    //
    let list_response = mist_client.streams().list().await?;

    info!(
        total_streams = list_response.streams.len(),
        streams = ?list_response.streams,
        "Retrieved stream list"
    );

    // ---------------------------------------------------------------------
    // Wait for demonstration purposes
    // ---------------------------------------------------------------------
    //
    // Give MistServer a few seconds to initialize newly created streams.
    // This delay is only included for demonstration and is generally not
    // required in production applications.
    //
    info!("Waiting for streams to initialize...");
    sleep(Duration::from_secs(5)).await;

    // ---------------------------------------------------------------------
    // Cleanup
    // ---------------------------------------------------------------------
    //
    // Remove only the streams created by this example.
    //
    cleanup_streams(&mut mist_client).await?;

    info!("Example completed successfully");

    Ok(())
}

/// Creates a single stream.
///
/// Demonstrates the simplest way to register a new stream using the builder
/// pattern.
async fn add_single_stream(mist_client: &mut MistClient) -> anyhow::Result<()> {
    info!("Creating stream 'stream_some'");

    // StreamBuilder uses the builder pattern so optional configuration can
    // be specified fluently.
    //
    // name:
    //     Unique identifier of the stream.
    //
    // source:
    //     Media source to ingest.
    //
    // always_on:
    //     Keep the stream active continuously.
    //
    // debug:
    //     Enable MistServer debug logging for this stream.
    //
    let stream = StreamBuilder::new("stream_some", "/video/file.mp4")
        .always_on(true)
        .debug(10)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build stream configuration: {e}"))?;

    mist_client
        .streams()
        .add(stream)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create stream: {e}"))?;

    info!("Successfully created stream 'stream_some'");

    Ok(())
}

/// Creates multiple streams.
///
/// Batch creation is typically more efficient than issuing several individual
/// API requests.
async fn add_multiple_streams(mist_client: &mut MistClient) -> anyhow::Result<()> {
    info!("Preparing multiple stream configurations");

    let stream_configs = vec![
        ("stream1", "/video/file.mp4"),
        ("stream2", "/video/file.mp4"),
        ("stream3", "/video/other.mp4"),
    ];

    // add_many() expects a mapping between stream names and stream
    // configurations.
    let mut streams = HashMap::with_capacity(stream_configs.len());

    for (name, source) in stream_configs {
        match StreamBuilder::new(name, source)
            .always_on(true)
            .debug(5)
            .build()
        {
            Ok(stream) => {
                info!(stream = name, "Prepared stream configuration");
                streams.insert(name.to_string(), stream);
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
        .map_err(|e| anyhow::anyhow!("Failed to create streams: {e}"))?;

    info!("Successfully created multiple streams");

    Ok(())
}

/// Deletes the streams created during this example.
///
/// Production applications typically delete streams only when they are no
/// longer needed.
async fn cleanup_streams(mist_client: &mut MistClient) -> anyhow::Result<()> {
    let streams_to_delete = vec![
        "stream_some".to_string(),
        "stream1".to_string(),
        "stream2".to_string(),
        "stream3".to_string(),
    ];

    info!(
        streams = ?streams_to_delete,
        "Cleaning up example streams"
    );

    mist_client
        .streams()
        .delete(streams_to_delete)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to delete streams: {e}"))?;

    info!("Cleanup completed");

    // The client remains fully usable after cleanup. Additional API requests
    // can be issued until the client is dropped.

    Ok(())
}
