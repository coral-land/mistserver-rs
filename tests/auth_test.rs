use mistserver_rs::{
    AuthResult, MistAuthController, Result, config::Config, http::client::build_http_client,
};

use std::{sync::Arc, time::Duration};

#[tokio::test]
async fn auth_without_authorization() -> Result<()> {
    let client = Arc::new(build_http_client(Duration::from_secs(10))?);
    let config = Arc::new(Config {
        mist_url: "http://localhost:8080".into(),
        auth: None,
    });

    let mac = MistAuthController::new(client.clone(), config.clone());
    let auth_result: AuthResult = mac.authorize().await.unwrap();

    match auth_result {
        AuthResult::NotRequired => {}
        AuthResult::Required(_) => panic!("Expected NotRequired, got Required"),
    }

    assert!(matches!(auth_result, AuthResult::NotRequired));
    Ok(())
}
