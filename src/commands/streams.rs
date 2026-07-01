use crate::models::Stream;
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
