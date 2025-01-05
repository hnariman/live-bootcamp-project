use crate::helpers::TestApp;

#[tokio::test]
async fn verify_token_is_alive() {
    let app = TestApp::new().await;
    let response = app.post_route("/verify-token").await;
    assert_eq!(response.status().as_u16(), 200);
}
