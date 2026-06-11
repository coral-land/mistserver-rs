pub struct Config {
    pub mist_api_url: String,
    pub auth: Option<(String, String)>,
}

impl Config {
    pub fn auth_enabled(&self) -> bool {
        self.auth.is_some()
    }
}
