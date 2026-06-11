use crate::Result;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use url::Url;

pub struct MistApi<C>
where
    C: Send + Sync + Serialize + DeserializeOwned,
{
    mist_api_url: String,
    client: Arc<Client>,
    command: C,
}

impl<C> MistApi<C>
where
    C: Send + Sync + Serialize + DeserializeOwned,
{
    pub(crate) fn new(mist_api_url: String, client: Arc<Client>, command: C) -> Self {
        Self {
            mist_api_url,
            client,
            command,
        }
    }

    pub(crate) async fn send_command<T>(&self) -> Result<T>
    where
        T: Send + Sync + Serialize + DeserializeOwned,
    {
        let mut request_url = Url::parse(&self.mist_api_url)?;
        let command = serde_json::to_string(&self.command)?;

        request_url
            .query_pairs_mut()
            .append_pair("command", &command);

        let response = self.client.get(request_url).send().await?;
        let auth_response: T = response.json().await?;
        let response = self.client.get(&self.mist_api_url).send().await?;

        Ok(response.json::<T>().await?)
    }
}
