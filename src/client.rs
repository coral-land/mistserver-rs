use crate::{auth::MistAuthController, utils::build_http_client};
use reqwest::Client;
use std::{sync::Arc, time::Duration};

pub struct MistClient {
    mist_api_url: String,
    auth: Option<(String, String)>,
    client: Arc<Client>,
    auth_controller: Option<MistAuthController>,
}

impl MistClient {
    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn auth_enabled(&self) -> bool {
        self.auth.is_some()
    }

    pub fn auth_credentials(&self) -> Option<(String, String)> {
        self.auth.clone()
    }
}

pub struct MistClientBuilder {
    inner: MistClient,
}

impl MistClientBuilder {
    pub fn new(base_api_url: &str, client: Option<Client>) -> Self {
        let client = Arc::new(client.clone().take().unwrap_or_else(|| {
            build_http_client(Duration::from_secs(10)).expect("Can not construct default client")
        }));

        Self {
            inner: MistClient {
                mist_api_url: base_api_url.into(),
                auth: None,
                client,
                auth_controller: None,
            },
        }
    }

    pub fn with_auth(mut self, username: &str, password: &str) -> Self {
        self.inner.auth = Some((username.into(), password.into()));
        self
    }

    pub fn with_client(mut self, client: Arc<reqwest::Client>) -> Self {
        self.inner.client = client;
        self
    }

    pub fn build(mut self) -> MistClient {
        self.inner.auth_controller = Some(MistAuthController::new(
            self.inner.client.clone(),
            self.inner.mist_api_url.clone(),
            self.inner.auth.clone(),
        ));

        self.inner
    }
}
