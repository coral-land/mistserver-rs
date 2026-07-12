use crate::{MistClient, Protocol, Result, commands::protocols::AddProtocolCommmand};

pub struct ProtocolsController<'a> {
    client: &'a MistClient,
}

impl<'a> ProtocolsController<'a> {
    pub fn new(client: &'a MistClient) -> Self {
        Self { client }
    }

    pub async fn add(&self, protocols: Vec<Protocol>) -> Result<()> {
        let command = AddProtocolCommmand::new(protocols);
        self.client.execute(command).await
    }

    // TODO: remove
    // TODO: List
    // TODO: Update
}
