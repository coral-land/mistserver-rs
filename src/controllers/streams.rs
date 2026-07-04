//! Stream management controller for the Mist API.
//!
//! This module provides a high-level, ergonomic interface over the underlying
//! Mist stream commands. It acts as a thin orchestration layer that:
//!
//! - Translates method calls into strongly typed commands
//! - Delegates execution to the [`MistClient`]
//! - Returns structured responses defined by the SDK
//!
//! The controller itself contains no business logic; it exists purely to
//! improve usability and reduce direct command construction at call sites.

use crate::{
    MistClient, Result,
    commands::streams::{
        AddStreamCommand, DeleteStreamCommand, ListActiveStreamsCommand, ListActiveStreamsResponse,
        NukeStreamCommand, StreamCommandsResponse, StreamListCommand, TagStreamCommand, TagValue,
        UntagStreamCommand,
    },
    models::Stream,
};

use std::collections::HashMap;

/// High-level interface for managing streams in the Mist system.
///
/// Each method corresponds directly to a Mist API command but provides a
/// simplified and intention-revealing interface.
pub struct StreamController<'a> {
    /// Shared API client used to execute Mist commands.
    client: &'a MistClient,
}

impl<'a> StreamController<'a> {
    /// Creates a new stream controller bound to the given Mist client.
    ///
    /// The controller does not own the client and only borrows it.
    pub fn new(client: &'a MistClient) -> Self {
        Self { client }
    }

    /// Creates or updates a single stream.
    ///
    /// If a stream with the same name already exists, the Mist server will
    /// update it according to its internal merge/update rules.
    ///
    /// # Returns
    /// The server response containing information about the created or updated
    /// stream(s).
    pub async fn add(&self, stream: Stream) -> Result<StreamCommandsResponse> {
        let command = AddStreamCommand::from(stream);
        self.client.execute(command).await
    }

    /// Creates or updates multiple streams in a single request.
    ///
    /// This is more efficient than calling [`add`](Self::add) repeatedly when
    /// working with batch stream creation or updates.
    ///
    /// # Returns
    /// A response containing the created or updated stream definitions.
    pub async fn add_many(
        &self,
        streams: HashMap<String, Stream>,
    ) -> Result<StreamCommandsResponse> {
        let command = AddStreamCommand::from(streams);
        self.client.execute(command).await
    }

    /// Retrieves all configured streams from the Mist server.
    ///
    /// This includes both active and inactive streams, depending on server
    /// configuration and state.
    ///
    /// # Returns
    /// A map of stream names to their configuration details.
    pub async fn list(&self) -> Result<StreamCommandsResponse> {
        let command = StreamListCommand { streams: () };
        self.client.execute(command).await
    }

    /// Retrieves real-time statistics for all active streams.
    ///
    /// Only streams that are currently active are included in the response.
    /// Metrics may include viewer counts, bandwidth usage, and health data.
    ///
    /// # Returns
    /// A structured response containing active stream metrics.
    pub async fn list_active(&self) -> Result<ListActiveStreamsResponse> {
        let command = ListActiveStreamsCommand::new();
        self.client.execute(command).await
    }

    /// Deletes one or more streams by name.
    ///
    /// This operation removes the stream configuration from the Mist server.
    ///
    /// # Arguments
    /// * `names` - Names of the streams to delete.
    ///
    /// # Returns
    /// Returns `Ok(())` if the server accepted the deletion request.
    pub async fn delete(&self, names: Vec<String>) -> Result<()> {
        let command = DeleteStreamCommand::new(names);
        self.client.execute(command).await?;
        Ok(())
    }

    /// Forcefully removes a stream and cleans up all associated runtime state.
    ///
    /// This is a destructive operation that ensures the stream is fully shut
    /// down and any remaining resources are released by the server.
    ///
    /// # Arguments
    /// * `name` - Name of the stream to remove.
    ///
    /// # Returns
    /// Returns `Ok(())` if the operation succeeds.
    pub async fn nuke_stream(&self, name: String) -> Result<()> {
        let command = NukeStreamCommand::new(name);
        self.client.execute(command).await?;

        Ok(())
    }

    /// This request allows you to set a specific tag on a stream.
    /// They are not in any way related to session tags. A stream tag
    /// can be used to automatically start pushes or triggers depending on the tag.
    /// Which allows you to set up different workflows for streams in the same wildcard group.
    /// For example only adding recording for "some" of the current live streams.
    ///
    /// # Returns
    /// Error or nothing
    ///
    pub async fn tag(&self, stream: &str, tags: Vec<&str>) -> Result<()> {
        // TODO: Add a builder to add tags
        let tags = TagValue::from(tags);
        let mut map: HashMap<String, TagValue> = HashMap::new();

        map.insert(stream.into(), tags);

        let command = TagStreamCommand::new(map);
        self.client.execute(command).await?;

        Ok(())
    }

    /// This request allows you to set a specific tag on a stream.
    /// They are not in any way related to session tags. A stream tag
    /// can be used to automatically start pushes or triggers depending on the tag.
    /// Which allows you to set up different workflows for streams in the same wildcard group.
    /// For example only adding recording for "some" of the current live streams.
    ///
    /// # Returns
    /// Error or nothing
    pub async fn untag(&self, stream: &str, tags: Vec<&str>) -> Result<()> {
        // TODO: Add a builder to add tags
        let tags = TagValue::from(tags);
        let mut map: HashMap<String, TagValue> = HashMap::new();

        map.insert(stream.into(), tags);

        let command = UntagStreamCommand::new(map);
        self.client.execute(command).await?;

        Ok(())
    }
}
