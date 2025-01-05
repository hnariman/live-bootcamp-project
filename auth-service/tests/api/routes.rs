use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;
    let response = app.get_root().await;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_is_alive() {
    let app = TestApp::new().await;
    let response = app.post_route("/signup").await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_is_alive() {
    let app = TestApp::new().await;
    let response = app.post_route("/login").await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_is_alive() {
    let app = TestApp::new().await;
    let response = app.post_route("/verify-2fa").await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_is_alive() {
    let app = TestApp::new().await;
    let response = app.post_route("/logout").await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_is_alive() {
    let app = TestApp::new().await;
    let response = app.post_route("/verify-token").await;
    assert_eq!(response.status().as_u16(), 200);
}
