#[cfg(test)]
mod test {
    use mistserver_rs::{MistClientBuilder, Result};
    use reqwest::Client;
    use std::sync::Arc;

    pub fn test_client() -> Client {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap()
    }

    #[tokio::test]
    pub async fn mist_client_builder_with_auth_sets_auth_controller() -> Result<()> {
        let http_client = Arc::new(test_client());

        let mist_client = MistClientBuilder::new("http://localhost:8080")
            .with_client(http_client)
            .with_auth("admin", "password")
            .build();

        assert!(mist_client.auth_enabled());

        mist_client.auth_credentials().map(|(username, password)| {
            assert_eq!(username, "admin");
            assert_eq!(password, "password");
        });

        Ok(())
    }

    #[tokio::test]
    pub async fn mist_client_builder_with_client_sets_client() -> Result<()> {
        let http_client = Arc::new(test_client());

        let mist_client = MistClientBuilder::new("http://localhost:8080")
            .with_client(http_client.clone())
            .build();

        let client_arc = mist_client.client();

        assert!(std::ptr::eq(
            Arc::as_ptr(&client_arc),
            Arc::as_ptr(&http_client)
        ));

        Ok(())
    }
}
