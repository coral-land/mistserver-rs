use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

use crate::StreamInfo;

pub fn deserialize_streams_map<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<String, StreamInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let raw: HashMap<String, serde_json::Value> = HashMap::deserialize(deserializer)?;
    let mut result = HashMap::new();
    for (key, value) in raw {
        if key == "incomplete list" {
            continue;
        }
        let info: StreamInfo = serde_json::from_value(value).map_err(Error::custom)?;
        result.insert(key, info);
    }
    Ok(result)
}
