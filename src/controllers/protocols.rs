use crate::MistClient;

pub struct ProtocolsController<'a> {
    client: &'a MistClient,
}

impl<'a> ProtocolsController<'a> {
    async fn new(client: &'a MistClient) -> Self {
        Self { client }
    }

    // TODO: Add
    // TODO: remove
    // TODO: List
    // TODO: Update
}
