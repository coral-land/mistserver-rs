use crate::{Result, commands::traits::MistCommand};

pub trait Transport {
    async fn execute<C: MistCommand>(&self, command: C) -> Result<C::Response>;
}
