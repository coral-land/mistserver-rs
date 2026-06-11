use std::sync::Arc;

use mistserver_rs::{MistClientBuilder, Result};

#[tokio::test]
pub async fn mist_client_builder_with_auth_sets_auth_controller() -> Result<()> {
    let client = MistClientBuilder::new("http://localhost:8080", None)
        .with_auth("admin", "password")
        .build();

    assert!(client.auth_enabled());

    client.auth_credentials().map(|(username, password)| {
        assert_eq!(username, "admin");
        assert_eq!(password, "password");
    });

    Ok(())
}

#[tokio::test]
pub async fn mist_client_builder_with_client_sets_client() -> Result<()> {
    use std::sync::Arc;

    let custom_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap();
    let custom_client_arc = Arc::new(custom_client);

    let mist_client = MistClientBuilder::new("http://localhost:8080", None)
        .with_client(custom_client_arc.clone())
        .build();

    let client_arc = mist_client.client();

    assert!(std::ptr::eq(
        Arc::as_ptr(&client_arc),
        Arc::as_ptr(&custom_client_arc)
    ));

    Ok(())
}
