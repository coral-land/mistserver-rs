use crate::{MistClient, Protocol};

pub struct ProtocolsController<'a> {
    client: &'a MistClient,
}

impl<'a> ProtocolsController<'a> {
    pub fn new(client: &'a MistClient) -> Self {
        Self { client }
    }

    // TODO: Add
    pub async fn add(&self, protocol: Protocol) {
        todo!()
    }

    // TODO: remove
    // TODO: List
    // TODO: Update
}
