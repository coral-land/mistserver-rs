use crate::ProtocolConnector;

pub struct ProtocolBuilder {
    connector: ProtocolConnector,
}

impl ProtocolBuilder {
    pub fn new(connector: ProtocolConnector) -> Self {
        Self { connector }
    }
}
