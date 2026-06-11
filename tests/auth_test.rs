use std::{sync::Arc, time::Duration};

use mistserver_rs::{
    Result,
    auth::{AuthResponse, AuthResponseWrapper, AuthResult, AuthStatus, MistAuthController},
    models::Config,
    utils::build_http_client,
};

#[tokio::test]
async fn auth_without_authorization() -> Result<()> {
    let client = Arc::new(build_http_client(Duration::from_secs(10))?);
    let config = Arc::new(Config {
        mist_api_url: "http://localhost:8080/api".into(),
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

#[tokio::test]
async fn auth_with_challenge() -> Result<()> {
    let client = Arc::new(build_http_client(Duration::from_secs(10))?);
    let config = Arc::new(Config {
        mist_api_url: "http://localhost:1324/api".into(),
        auth: Some(("admin".into(), "password".into())),
    });

    let srv_opts = mockito::ServerOpts {
        host: "0.0.0.0",
        port: 1324,
        assert_on_drop: false,
        ..Default::default()
    };

    let mut server = mockito::Server::new_with_opts_async(srv_opts).await;

    let challenge_response = serde_json::to_string(&AuthResponseWrapper {
        authorize: AuthResponse {
            status: Some(AuthStatus::Chall),
            challenge: Some("challenge_str".into()),
        },
    })?;

    server
        .mock("GET", "/api")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(challenge_response)
        .match_query(mockito::Matcher::Any)
        .create_async()
        .await;

    let mac = MistAuthController::new(client.clone(), config.clone());
    let auth_result = mac.authorize().await.unwrap();

    match auth_result {
        AuthResult::NotRequired => panic!("Expected Required, got NotRequired"),
        AuthResult::Required(_) => {
            assert_eq!(
                auth_result,
                AuthResult::Required(AuthResponse {
                    status: Some(AuthStatus::Chall),
                    challenge: Some("challenge_str".into()),
                })
            );
        }
    }

    Ok(())
}
