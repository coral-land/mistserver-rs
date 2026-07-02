//! This module defines the `MistCommand` trait, which is used to represent commands that can be sent to the MistServer API.

use serde::{Serialize, de::DeserializeOwned};

pub trait MistCommand: Sync + Send + Serialize {
    /// Response you will get from the MistServer API after executing this command.
    type Response: Sync + Send + DeserializeOwned;

    /// The name of the command as expected by the MistServer API.
    /// This is used to identify the command when sending requests to the API.
    const NAME: &'static str;
}
