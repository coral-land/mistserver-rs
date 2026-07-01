use crate::{
    StreamInfo,
    commands::{traits::MistCommand, utils::deserialize_streams_map},
    models::Stream,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Command payload for adding one or more streams.
///
/// This struct is serialized into JSON and sent as the `addstream` command
/// to the Mist API. The keys are stream names and the values are stream
/// configurations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAddCommand {
    pub addstream: HashMap<String, Stream>,
}

impl StreamAddCommand {
    /// Creates a new `StreamAddCommand` with the given stream configurations.
    ///
    /// # Arguments
    /// * `streams` - A map from stream names to their `Stream` configurations.
    pub fn new(streams: HashMap<String, Stream>) -> Self {
        Self { addstream: streams }
    }
}

/// Implementation of MistCommand
impl MistCommand for StreamAddCommand {
    type Response = StreamAddCommandResponse;
    const NAME: &'static str = "addstream";
}

/// Response received after successfully adding streams.
///
/// Contains a map of stream names to their detailed information as returned
/// by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAddCommandResponse {
    /// Map of stream names to their `StreamInfo` details.
    #[serde(deserialize_with = "deserialize_streams_map")]
    pub streams: HashMap<String, StreamInfo>,
}

/// Command for deleting one or more streams.
///
/// The Mist API accepts multiple formats for deletion:
/// - A single stream name as a string.
/// - An array of stream names.
/// - A more complex object (hash map) for advanced deletion criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStreamCommand {
    // Stream name list
    pub deletestream: Vec<String>,
}

impl DeleteStreamCommand {
    /// Creates a new `DeleteStreamCommand` with the given stream names.
    ///
    /// # Arguments
    /// * `names` - A vector of stream names to be deleted.
    pub fn new(names: Vec<String>) -> Self {
        Self {
            deletestream: names,
        }
    }
}

impl MistCommand for DeleteStreamCommand {
    type Response = ();
    const NAME: &'static str = "deletestream";
}
