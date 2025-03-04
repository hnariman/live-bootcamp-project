use auth_service::utils::constants::JWT_COOKIE_NAME;

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
        "email": "exis"
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
        "email": "bestie@mail.com"
    });

    let response = app.post_login(&test_case).await;
    assert_eq!(response.status().as_u16(), 401)
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    // let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": "e1xisting@user.com",
        "password": "!@#(*$&#!234234alsdkj!@#",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": "e1xisting@user.com",
        "password": "!@#(*$&#!234234alsdkj!@#",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}
