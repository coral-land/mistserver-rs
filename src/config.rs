use super::models::Config;

pub struct ConfigBuilder {
    inner: Config,
}

impl ConfigBuilder {
    pub fn new(base_api_url: &str) -> Self {
        Self {
            inner: Config {
                mist_api_url: base_api_url.into(),
                auth: None,
            },
        }
    }

    pub fn with_auth(mut self, username: &str, password: &str) -> Self {
        self.inner.auth = Some((username.into(), password.into()));
        self
    }

    pub fn build(self) -> Config {
        self.inner
    }
}
