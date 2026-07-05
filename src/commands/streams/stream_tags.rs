use crate::commands::traits::MistCommand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamTagsCommand {
    stream_tags: Vec<String>,
}

impl StreamTagsCommand {
    #[must_use]
    pub fn new(names: Vec<String>) -> Self {
        Self { stream_tags: names }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamTagsCommandResponse {
    stream_tags: HashMap<String, Vec<String>>,
}

impl MistCommand for StreamTagsCommand {
    type Response = StreamTagsCommandResponse;
    const NAME: &'static str = "stream_tags";
}
