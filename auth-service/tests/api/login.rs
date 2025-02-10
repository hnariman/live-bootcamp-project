use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_happy_path_if_user_found() {
    let app = TestApp::new().await;

    let test_case = serde_json::json!({
        "password": "!@#(*$&#!234234alsdkj!@#",
        "email": "existing@user.com"
    });

    let response = app.post_login(&test_case).await;
    dbg!(&response);
    assert_eq!(response.status().as_u16(), 200)
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let test_case = serde_json::json!({
        "password": "!@#(*$&#!234234alsdkj!@#",
        "email": "exis@user.com"
    });

    let response = app.post_login(&test_case).await;
    assert_eq!(response.status().as_u16(), 422)
}

#[tokio::test]
async fn should_return_400_if_invalid_creds() {
    let app = TestApp::new().await;

    let test_case = "";

    let response = app.post_login(&test_case).await;
    assert_eq!(response.status().as_u16(), 422)
}

#[tokio::test]
async fn should_return_401_if_creds_are_valid_but_incorrect() {
    let app = TestApp::new().await;

    let test_case = serde_json::json!({
        "password": "!@#(*$&#!234234alsdkj!@#",
        "email": ""
    });

    let response = app.post_login(&test_case).await;
    assert_eq!(response.status().as_u16(), 401)
}

// If JSON is missing or malformed -> 422
// If JSON contains invalid credentials -> 400
// If JSON contains credentials that are valid but incorrect -> 401
