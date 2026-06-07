use std::env;

pub struct Config {
    pub mist_url: String,
    pub auth: Option<(String, String)>,
}

impl Config {
    pub fn auth_enabled(&self) -> bool {
        self.auth.is_some()
    }
}

pub fn config() -> Config {
    Config {
        mist_url: get_env("MIST_URL", "http://localhost:8080"),
        auth: get_auth(),
    }
}

pub fn get_auth() -> Option<(String, String)> {
    match (env::var("MIST_USER"), env::var("MIST_PASS")) {
        (Ok(user), Ok(pass)) => Some((user, pass)),
        _ => None,
    }
}

fn get_env(key: &'static str, default: &'static str) -> String {
    match env::var(key) {
        Err(_) => default.into(),
        Ok(env_val) => env_val.into(),
    }
}
