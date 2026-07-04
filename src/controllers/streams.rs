//! Stream management controller for the Mist API.
//!
//! This module provides functionality to manage streams, including creating
//! and deleting streams via the Mist API. It defines the command structures
//! and the controller that interacts with the API.
use crate::{
    MistClient, Result,
    commands::streams::{
        AddStreamCommand, DeleteStreamCommand, ListActiveStreamsCommand, ListActiveStreamsResponse,
        NukeStreamCommand, StreamCommandsResponse, StreamListCommand,
    },
    models::Stream,
};

use std::collections::HashMap;

/// Controller for managing streams via the Mist API.
///
/// Provides methods to perform operations on streams such as creating new ones.
pub struct StreamController<'a> {
    client: &'a MistClient,
}

impl<'a> StreamController<'a> {
    /// Creates a new `StreamsController` with the given API handle.
    pub fn new(client: &'a MistClient) -> Self {
        Self { client }
    }

    /// Create one single stream
    /// This will update if the stream with same name exists based on the mist server api
    ///
    /// # Returns
    /// A `Result` containing the `StreamAddResponse` with details of the created streams.
    pub async fn add(&self, stream: Stream) -> Result<StreamCommandsResponse> {
        let command = AddStreamCommand::from(stream);
        self.client.execute(command).await
    }

    /// Creates many streams with your options
    /// This will update the stream with same name if exists.
    ///
    /// # Returns
    /// A `Result` containing the StreamAddResponse with details of created stream.
    pub async fn add_many(
        &self,
        streams: HashMap<String, Stream>,
    ) -> Result<StreamCommandsResponse> {
        let command = AddStreamCommand::from(streams);
        self.client.execute(command).await
    }

    /// Get all streams in one go.
    /// It can not be so effective.
    ///
    /// # Returns
    /// - A `StreamCommandsResponse` containing the streams HashMap.
    pub async fn list(&self) -> Result<StreamCommandsResponse> {
        let command = StreamListCommand { streams: () };
        self.client.execute(command).await
    }

    /// List Active Streams with metrics
    /// This requests a list of streams that are currently active, and only those.
    /// The list includes any wildcard versions of streams as well as temporary streams that may be active.
    ///
    /// # Returns
    /// A response containing the active streams with it's stats
    pub async fn list_active(&self) -> Result<ListActiveStreamsResponse> {
        let command = ListActiveStreamsCommand::new();
        self.client.execute(command).await
    }

    /// Deletes one or more streams by their names.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the delete operation.
    /// You will get Ok() if the delete operation was successful, or an error if it failed.
    pub async fn delete(&self, names: Vec<String>) -> Result<()> {
        let command = DeleteStreamCommand::new(names);
        let result = self.client.execute(command).await?;

        Ok(())
    }

    /// This call can shut down a running stream completely and/or clean up any potentially
    /// left over stream data in memory. It attempts a clean shutdown of the running stream first,
    /// followed by a forced shut down, and then follows up by checking for left over data in memory
    /// and cleaning that up if any is found.
    ///
    /// # Returns
    /// There is no response for this method
    pub async fn nuke_stream(&self, name: String) -> Result<()> {
        let command = NukeStreamCommand::new(name);
        self.client.execute(command).await
    }
}
