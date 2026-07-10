use serde::{Deserialize, Serialize};
use crate::ProtocolConnector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddProtocolCommmand {
    pub connector: ProtocolConnector,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub cert: Option<String>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub ctrlprefix: Option<String>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub debug: Option<i8>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub default_track_sorting: Option<String>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub iceservers: Option<String>
}


