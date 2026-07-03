//! Stream management controller for the Mist API.
//!
//! This module provides functionality to manage streams, including creating
//! and deleting streams via the Mist API. It defines the command structures
//! and the controller that interacts with the API.

use crate::{
    MistClient, Result,
    commands::streams::{DeleteStreamCommand, StreamAddCommand, StreamAddCommandResponse},
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
    pub async fn add(&self, stream: Stream) -> Result<StreamAddCommandResponse> {
        let command = StreamAddCommand::from(stream);
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
    ) -> Result<StreamAddCommandResponse> {
        let command = StreamAddCommand::from(streams);
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
}
