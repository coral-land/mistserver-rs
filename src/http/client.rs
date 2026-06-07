use std::time::Duration;

use crate::Result;
use reqwest::Client;

pub fn build_http_client(timeout: Duration) -> Result<Client> {
    let client = Client::builder().http1_only().timeout(timeout).build()?;
    Ok(client)
}
