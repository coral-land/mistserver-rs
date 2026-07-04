use std::collections::HashMap;

use tracing::debug;

use crate::{MistError, Result, Stream};

/// Fluent builder for creating a `Stream` configuration.
#[derive(Default, Debug)]
pub struct StreamBuilder {
    source: String,
    name: String,
    always_on: Option<bool>,
    buffer_time: Option<i32>,
    debug: Option<i32>,
    fallback_stream: Option<String>,
    extra: Option<HashMap<String, serde_json::Value>>,
}

impl StreamBuilder {
    /// Start with a mandatory `source` (required).
    pub fn new(name: &str, source: &str) -> Self {
        Self {
            source: source.into(),
            name: name.into(),
            ..Default::default()
        }
    }

    /// [Optional] Set a name for the stream (not actually used by the API,
    /// included for consistency; the name is the key in the `HashMap` when adding streams).
    pub fn name(mut self, value: &'static str) -> Self {
        self.name = value.into();
        self
    }

    /// Keep the stream active even with no viewers (default: `false`).
    pub fn always_on(mut self, value: bool) -> Self {
        self.always_on = Some(value);
        self
    }

    /// Buffer duration in milliseconds for live streams (default: `50000`).
    pub fn buffer_time(mut self, value: i32) -> Self {
        self.buffer_time = Some(value);
        self
    }

    /// Debug verbosity level (1‑6, default: `3`).
    pub fn debug(mut self, value: i32) -> Self {
        self.debug = Some(value);
        self
    }

    /// Fallback stream name if this one cannot be opened.
    pub fn fallback_stream(mut self, value: &'static str) -> Self {
        self.fallback_stream = Some(value.into());
        self
    }

    /// Add any extra parameters (e.g. `username`, `password`, `cut_time`).
    pub fn extra(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.extra = Some(value);
        self
    }

    /// This will validates the stream then if everything is ok
    ///
    /// # returns
    /// If everything is good, `Ok()` else `Err(MistError::Validation(...))`
    pub(crate) fn validate(stream: &Stream) -> Result<()> {
        debug!(stream = %stream.name, "Validating stream");

        if stream.name.len() > 100 {
            return Err(MistError::Validation {
                target: "stream".into(),
                name: stream.name.clone(),
                error: "Stream name can not be more than 100 char".into(),
            });
        }

        if !stream
            .name
            .clone()
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(MistError::Validation {
                target: "stream".into(),
                name: stream.name.clone(),
                error:
                    "only lower case letters a-z, numbers, underscores _, dashes - and periods ."
                        .into(),
            });
        }

        debug!(stream = %stream.name, "Stream is valid and ready to be created");
        Ok(())
    }

    /// Build the final `Stream` object.
    pub fn build(self) -> Result<Stream> {
        let stream = Stream {
            name: self.name,
            source: self.source,
            always_on: self.always_on,
            buffer_time: self.buffer_time,
            debug: self.debug,
            fallback_stream: self.fallback_stream,
            extra: self.extra,
        };

        Self::validate(&stream)?;
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    // ------------------------------------------------------------------------
    // Builder tests
    // ------------------------------------------------------------------------

    use std::collections::HashMap;

    use serde_json::json;

    use crate::{Result, StreamBuilder};

    #[test]
    fn builder_defaults() -> Result<()> {
        let stream = StreamBuilder::new("name", "push://").build()?;
        assert_eq!(stream.source, "push://");
        assert!(stream.always_on.is_none());
        assert!(stream.buffer_time.is_none());
        assert!(stream.debug.is_none());
        assert!(stream.fallback_stream.is_none());
        assert!(stream.extra.is_none());

        Ok(())
    }

    #[test]
    fn builder_set_all_fields() -> Result<()> {
        let mut extra = HashMap::new();
        extra.insert("cut_time".to_string(), json!(0));
        extra.insert("segment_size".to_string(), json!(6000));

        let stream = StreamBuilder::new("name", "dtsc://1.2.3.4/video")
            .name("ignored")
            .always_on(true)
            .buffer_time(30000)
            .debug(4)
            .fallback_stream("backup")
            .extra(extra.clone())
            .build()?;

        assert_eq!(stream.source, "dtsc://1.2.3.4/video");
        assert_eq!(stream.always_on, Some(true));
        assert_eq!(stream.buffer_time, Some(30000));
        assert_eq!(stream.debug, Some(4));
        assert_eq!(stream.fallback_stream, Some("backup".to_string()));
        assert_eq!(stream.extra, Some(extra));

        Ok(())
    }

    #[test]
    fn builder_partial_fields() -> Result<()> {
        let stream = StreamBuilder::new("name", "file:///media/video.mp4")
            .always_on(false)
            .debug(3)
            .build()?;

        assert_eq!(stream.source, "file:///media/video.mp4");
        assert_eq!(stream.always_on, Some(false));
        assert!(stream.buffer_time.is_none());
        assert_eq!(stream.debug, Some(3));
        assert!(stream.fallback_stream.is_none());
        assert!(stream.extra.is_none());

        Ok(())
    }
}
