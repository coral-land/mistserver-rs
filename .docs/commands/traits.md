# Commands traits
For now we only have one trait that contains a `Response` type and a constant called `NAME` witch is used to define the command name for future use.

## How to define one command:
In fact it's so simple, first import the `Command` trait then implement it to your specific struct as you can see in the example:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAddCommand {
    pub addstream: HashMap<String, Stream>,
}

impl StreamAddCommand {
    pub fn new(streams: HashMap<String, Stream>) -> Self {
        Self { addstream: streams }
    }
}

impl MistCommand for StreamAddCommand {
    type Response = StreamAddCommandResponse; // Define response type here
    const NAME: &'static str = "addstream";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAddCommandResponse {
    pub streams: HashMap<String, StreamInfo>,
}
```
