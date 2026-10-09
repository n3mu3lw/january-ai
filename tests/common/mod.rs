use january_ai::JanuaryAI;
use wiremock::MockServer;

pub const AUTH_HEADER: &str = "Bearer sk_test";

pub async fn setup_mock_client() -> (MockServer, JanuaryAI) {
    let mock_server = MockServer::start().await;

    let client = JanuaryAI::new("sk_test")
        .expect("failed to build JanuaryAI client")
        .with_base_url(mock_server.uri());

    (mock_server, client)
}
