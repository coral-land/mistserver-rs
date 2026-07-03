use mistserver_rs::{MistClient, MistClientBuilder, StreamBuilder};
use reqwest::Client;
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging with better configuration
    tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_level(true)
        .init();

    info!("Starting MistServer client example");

    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    let mut mist_client = MistClientBuilder::new("http://localhost:4242/api")
        .with_client(client)
        .with_auth("admin", "password")
        .build();

    // Authorize with better error handling
    mist_client
        .auth()
        .authorize()
        .await
        .map_err(|e| anyhow::anyhow!("Authorization failed: {}", e))?;

    info!("Authorization successful");

    // Add a single stream with proper error handling
    add_single_stream(&mut mist_client).await?;

    // Add multiple streams
    add_multiple_streams(&mut mist_client).await?;

    // Wait for streams to stabilize
    info!("Waiting 5 seconds for streams to initialize...");
    sleep(Duration::from_secs(5)).await;

    // Clean up streams
    cleanup_streams(&mut mist_client).await?;

    info!("Example completed successfully");
    Ok(())
}

async fn add_single_stream(mist_client: &mut MistClient) -> anyhow::Result<()> {
    let stream = StreamBuilder::new("stream_some", "/video/file.mp4")
        .always_on(true)
        .debug(10)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build stream: {}", e))?;

    match mist_client.streams().add(stream).await {
        Ok(response) => {
            info!("Stream 'stream_some' added successfully: {:?}", response);
            Ok(())
        }
        Err(e) => {
            error!("Failed to add stream 'stream_some': {}", e);
            Err(anyhow::anyhow!("Failed to add stream: {}", e))
        }
    }
}

async fn add_multiple_streams(mist_client: &mut MistClient) -> anyhow::Result<()> {
    // Use a vector for better iteration
    let stream_configs = vec![
        ("stream1", "/video/file.mp4"),
        ("stream2", "/video/file.mp4"),
        ("stream3", "/video/other.mp4"),
    ];

    let mut streams = HashMap::with_capacity(stream_configs.len());

    for (name, source) in stream_configs {
        match StreamBuilder::new(name, source)
            .always_on(true)
            .debug(5)
            .build()
        {
            Ok(stream) => {
                streams.insert(name.to_string(), stream);
                info!("Built stream configuration for '{}'", name);
            }
            Err(e) => {
                warn!("Failed to build stream '{}': {}", name, e);
            }
        }
    }

    if streams.is_empty() {
        return Err(anyhow::anyhow!("No valid stream configurations to add"));
    }

    info!("Adding {} streams to MistServer...", streams.len());

    match mist_client.streams().add_many(streams).await {
        Ok(response) => {
            info!("Multiple streams added successfully: {:?}", response);
            Ok(())
        }
        Err(e) => {
            error!("Failed to add multiple streams: {}", e);
            Err(anyhow::anyhow!("Failed to add multiple streams: {}", e))
        }
    }
}

async fn cleanup_streams(mist_client: &mut MistClient) -> anyhow::Result<()> {
    let streams_to_delete = vec!["stream1".to_string(), "stream_some".to_string()];

    info!("Cleaning up streams: {:?}", streams_to_delete);

    match mist_client.streams().delete(streams_to_delete).await {
        Ok(response) => {
            info!("Streams deleted successfully: {:?}", response);
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete streams: {}", e);
            Err(anyhow::anyhow!("Failed to delete streams: {}", e))
        }
    }
}
