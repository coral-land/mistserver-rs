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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize)]
    struct Wrapper {
        #[serde(deserialize_with = "deserialize_streams_map")]
        streams: HashMap<String, StreamInfo>,
    }

    #[test]
    fn deserialize_single_stream() {
        let json = json!({
            "streams": {
                "camera1": {
                    "name": "camera1",
                    "online": 1,
                    "source": "push://"
                }
            }
        });

        let wrapper: Wrapper = serde_json::from_value(json).unwrap();

        assert_eq!(wrapper.streams.len(), 1);
        assert!(wrapper.streams.contains_key("camera1"));
    }

    #[test]
    fn deserialize_empty_map() {
        let json = json!({
            "streams": {}
        });

        let wrapper: Wrapper = serde_json::from_value(json).unwrap();

        assert!(wrapper.streams.is_empty());
    }

    #[test]
    fn ignores_incomplete_list() {
        let json = json!({
            "streams": {
                "camera1": {
                    "name": "camera1",
                    "online": 1,
                    "source": "push://"
                },
                "incomplete list": true
            }
        });

        let wrapper: Wrapper = serde_json::from_value(json).unwrap();

        assert_eq!(wrapper.streams.len(), 1);
        assert!(wrapper.streams.contains_key("camera1"));
        assert!(!wrapper.streams.contains_key("incomplete list"));
    }

    #[test]
    fn invalid_stream_returns_error() {
        let json = json!({
            "streams": {
                "camera1": 42
            }
        });

        let result = serde_json::from_value::<Wrapper>(json);

        assert!(result.is_err());
    }

    #[test]
    fn only_incomplete_list_results_in_empty_map() {
        let json = json!({
            "streams": {
                "incomplete list": true
            }
        });

        let wrapper: Wrapper = serde_json::from_value(json).unwrap();

        assert!(wrapper.streams.is_empty());
    }
}
