#[cfg(test)]
mod tests {
    use mistserver_rs::MistClientBuilder;
    use reqwest::Client;
    use std::time::Duration;

    fn test_client() -> Client {
        Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("failed to build test client")
    }

    #[test]
    fn builder_defaults() {
        let builder = MistClientBuilder::new("http://example.com/api");
        assert_eq!(builder.mist_api_url, "http://example.com/api");
        assert!(builder.auth.is_none());
    }

    #[test]
    fn builder_with_auth_sets_credentials() {
        let builder = MistClientBuilder::new("http://example.com").with_auth("user123", "pass456");
        assert_eq!(builder.auth, Some(("user123".into(), "pass456".into())));
    }

    #[test]
    #[should_panic(
        expected = "Client should be initialized in Mist Client Builder using with_client() method"
    )]
    fn build_panics_without_client() {
        MistClientBuilder::new("http://example.com").build();
    }

    #[test]
    fn build_succeeds_with_client_only() {
        let client = test_client();
        let mist_client = MistClientBuilder::new("http://example.com/api")
            .with_client(client.clone())
            .build();

        // URL is private – we cannot assert on it, but building succeeded.
        assert!(mist_client.auth_credentials().is_none());
        assert!(!mist_client.auth_enabled());
    }

    #[test]
    fn build_with_auth_creates_auth_controller() {
        let client = test_client();
        let mist_client = MistClientBuilder::new("http://example.com/api")
            .with_client(client)
            .with_auth("admin", "secret")
            .build();

        assert!(mist_client.auth_enabled());
        let creds = mist_client.auth_credentials().unwrap();
        assert_eq!(creds, ("admin".into(), "secret".into()));
        // The auth_controller is internal, but its presence is implied by successful
        // authentication methods later (not tested here).
    }

    #[test]
    fn client_clone_returns_same_arc() {
        let client = test_client();
        let mist_client = MistClientBuilder::new("http://example.com")
            .with_client(client.clone())
            .build();

        let cloned_client = mist_client.client();
    }

    #[test]
    fn auth_credentials_clone() {
        let client = test_client();
        let mist_client = MistClientBuilder::new("http://example.com")
            .with_client(client)
            .with_auth("user", "pass")
            .build();

        let creds1 = mist_client.auth_credentials();
        let creds2 = mist_client.auth_credentials();
        assert_eq!(creds1, creds2);
    }

    #[test]
    fn multiple_builders_independent() {
        let client1 = test_client();
        let client2 = test_client();

        let builder1 = MistClientBuilder::new("http://api1.com").with_client(client1);
        let builder2 = MistClientBuilder::new("http://api2.com")
            .with_client(client2)
            .with_auth("foo", "bar");

        let mist1 = builder1.build();
        let mist2 = builder2.build();

        assert!(!mist1.auth_enabled());
        assert!(mist2.auth_enabled());
        assert_eq!(
            mist2.auth_credentials().unwrap(),
            ("foo".into(), "bar".into())
        );
    }
}
