use crate::Config;
use std::env;

pub fn config() -> Config {
    Config {
        mist_api_url: get_env("MIST_API_URL", "http://localhost:8080/api"),
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
        Ok(env_val) => env_val,
    }
}
