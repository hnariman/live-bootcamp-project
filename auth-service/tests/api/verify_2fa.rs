use crate::helpers::TestApp;
#[tokio::test]
async fn verify_2fa_is_alive() {
    let app = TestApp::new().await;
    let response = app.post_route("/verify-2fa").await;
    assert_eq!(response.status().as_u16(), 200);
}
