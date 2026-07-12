use crate::{Protocol, commands::traits::MistCommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddProtocolCommmand {
    addprotocols: Vec<Protocol>,
}

impl AddProtocolCommmand {
    pub fn new(protocols: Vec<Protocol>) -> Self {
        Self {
            addprotocols: protocols,
        }
    }
}

impl From<Protocol> for AddProtocolCommmand {
    fn from(value: Protocol) -> Self {
        Self {
            addprotocols: vec![value],
        }
    }
}

impl From<Vec<Protocol>> for AddProtocolCommmand {
    fn from(value: Vec<Protocol>) -> Self {
        Self {
            addprotocols: value,
        }
    }
}

impl MistCommand for AddProtocolCommmand {
    type Response = ();
    const NAME: &'static str = "addprotocol";
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::traits::MistCommand;

    fn protocol() -> Protocol {
        Protocol {
            connector: crate::ProtocolConnector::AAC,
            cert: None,
            ctrlprefix: None,
            debug: None,
            default_track_sorting: None,
            extra_fields: None,
            iceservers: None,
            online: None
        }
    }

    #[test]
    fn new_creates_command_with_protocols() {
        let protocols = vec![protocol(), protocol()];
        let command = AddProtocolCommmand::new(protocols.clone());

        assert_eq!(command.addprotocols, protocols);
    }

    #[test]
    fn from_single_protocol() {
        let protocol = protocol();
        let command = AddProtocolCommmand::from(protocol.clone());

        assert_eq!(command.addprotocols.len(), 1);
        assert_eq!(command.addprotocols[0], protocol);
    }

    #[test]
    fn from_vec_protocols() {
        let protocols = vec![protocol(), protocol()];
        let command = AddProtocolCommmand::from(protocols.clone());

        assert_eq!(command.addprotocols, protocols);
    }

    #[test]
    fn mist_command_name_is_correct() {
        assert_eq!(AddProtocolCommmand::NAME, "addprotocol");
    }

    #[test]
    fn mist_command_response_type_is_unit() {
        fn assert_response_type<C: MistCommand<Response = ()>>() {}

        assert_response_type::<AddProtocolCommmand>();
    }
}

