use crate::helpers::TestApp;
#[tokio::test]
async fn logout_is_alive() {
    let app = TestApp::new().await;
    let response = app.post_route("/logout").await;
    assert_eq!(response.status().as_u16(), 200);
}
