use crate::http::MistAuthController;
use reqwest::Client;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct MistClientBuilder {
    inner: MistClient,
    client: Option<Arc<Client>>,
}

impl MistClientBuilder {
    pub fn new(base_api_url: &str) -> Self {
        Self {
            inner: MistClient {
                mist_api_url: base_api_url.into(),
                ..Default::default()
            },
            client: None,
        }
    }

    pub fn with_auth(mut self, username: &str, password: &str) -> Self {
        self.inner.auth = Some((username.into(), password.into()));
        self
    }

    pub fn with_client(mut self, client: Arc<reqwest::Client>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(mut self) -> MistClient {
        let client = self.client.expect(
            "Client should be initialized in Mist Client Builder using with_client() method",
        );

        self.inner.client = client.clone();
        self.inner.auth_controller = Some(MistAuthController::new(
            client,
            self.inner.mist_api_url.clone(),
            self.inner.auth.clone(),
        ));

        self.inner
    }
}
