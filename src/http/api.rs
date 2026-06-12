use crate::Result;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use url::Url;

#[derive(Debug, Clone, Default)]
pub struct MistApi {
    mist_api_url: String,
    client: Arc<Client>,
}

impl MistApi {
    pub(crate) fn new(mist_api_url: String, client: Arc<Client>) -> Self {
        Self {
            mist_api_url,
            client,
        }
    }

    pub(crate) async fn send<T, C>(&self, command: C) -> Result<T>
    where
        T: Send + Sync + DeserializeOwned,
        C: Send + Sync + Serialize,
    {
        let mut request_url = Url::parse(&self.mist_api_url)?;
        let command = serde_json::to_string(&command)?;

        request_url
            .query_pairs_mut()
            .append_pair("command", &command);

        let response = self.client.get(request_url).send().await?;
        let auth_response: T = response.json().await?;
        let response = self.client.get(&self.mist_api_url).send().await?;

        Ok(response.json::<T>().await?)
    }
}
