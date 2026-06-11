use crate::Result;

pub fn build_http_client(timeout: std::time::Duration) -> Result<reqwest::Client> {
    Ok(reqwest::ClientBuilder::new()
        .http1_only()
        .http2_keep_alive_while_idle(true)
        .timeout(timeout)
        .build()?)
}
