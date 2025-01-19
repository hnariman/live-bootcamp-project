use crate::helpers::TestApp;

#[tokio::test]
async fn if_working() {
    let app = TestApp::new().await;
    let response = app.post_route("/verify-2fa").await;
    assert_eq!(response.status().as_u16(), 200);
}
