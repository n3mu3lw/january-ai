mod common;
use common::*;

use january_ai::{Error, MintClientTokenRequest, Scope};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

const TEST_USER_ID: &str = "testusr";

#[tokio::test]
async fn test_mint_client_token_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/mint_client_token_success.json");

    Mock::given(method("POST"))
        .and(path("/auth/client-tokens"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = MintClientTokenRequest {
        end_user_id: TEST_USER_ID.to_string(),
        scopes: vec![Scope::FoodsRead],
        ttl_seconds: None,
    };

    let res = client
        .mint_client_token(&req)
        .await
        .expect("Failed to mint client token");

    assert_eq!(res.expires_in, 1800);
    assert_eq!(res.end_user_id, TEST_USER_ID);
    assert_eq!(res.scopes, vec![Scope::FoodsRead]);
}

#[tokio::test]
async fn test_mint_client_token_invalid_request() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/mint_client_token_error.json");

    Mock::given(method("POST"))
        .and(path("/auth/client-tokens"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(400).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = MintClientTokenRequest {
        end_user_id: "".to_string(),
        scopes: vec![],
        ttl_seconds: None,
    };

    let err = client.mint_client_token(&req).await.unwrap_err();

    match err {
        Error::Api { code, message } => {
            assert_eq!(code, "invalid_request");
            assert!(!message.is_empty());
        }
        _ => panic!("Expected Error::Api variant, got {err:?}"),
    }
}

#[tokio::test]
async fn test_revoke_client_tokens_zero() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/revoke_client_tokens_zero.json");

    Mock::given(method("POST"))
        .and(path("/auth/client-token-revocations"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client
        .revoke_client_tokens(TEST_USER_ID)
        .await
        .expect("Failed to revoke client token(s)");

    assert_eq!(res.revoked_count, 0);
}

#[tokio::test]
async fn test_revoke_client_tokens_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/revoke_client_tokens_success.json");

    Mock::given(method("POST"))
        .and(path("/auth/client-token-revocations"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client
        .revoke_client_tokens(TEST_USER_ID)
        .await
        .expect("Failed to revoke client token(s)");

    assert_eq!(res.revoked_count, 1);
}
