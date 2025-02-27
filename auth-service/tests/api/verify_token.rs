use crate::helpers::{get_error, get_random_email, login, signup, TestApp};
use auth_service::{
    domain::{BannedTokenError, BannedTokenStore, User},
    utils::constants::JWT_COOKIE_NAME,
};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let test_case = serde_json::json!({ "token":"" });
    let response = app.post_verify_token(&test_case).await;

    assert_eq!(response.status().as_u16(), 422);
    assert_eq!(get_error(response).await, "Malformed token".to_owned())
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;
    let user = User::new(&get_random_email(), "!@#$)(*#!@#$987$#@!asdf", false).unwrap();
    signup(&app, &user).await;
    let login_res = login(&app, &user).await;

    let token = login_res
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found")
        .value()
        .to_string();

    let test_case = serde_json::json!({ "token": token });
    let response = app.post_verify_token(&test_case).await;

    assert_eq!(response.status().as_u16(), 200)
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let test_case = serde_json::json!({ "token":"321" });
    let _user = User::new(&get_random_email(), "!@#$)(*#!@#$987$#@!asdf", false).unwrap();

    let response = app.post_verify_token(&test_case).await;

    assert_eq!(response.status().as_u16(), 401);
    assert_eq!(get_error(response).await, "Invalid token".to_owned())
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;
    let user = User::new(&get_random_email(), "!@#$)(*#!@#$987$#@!asdf", false).unwrap();
    signup(&app, &user).await;
    let login_res = login(&app, &user).await;

    let token = login_res
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found")
        .value()
        .to_string();

    let _logout_res = app
        .post_logout(&serde_json::json!({ "email":user.email.as_ref() }))
        .await;

    let test_case = serde_json::json!({ "token": token });
    let response = app.post_verify_token(&test_case).await;

    assert_eq!(response.status().as_u16(), 401)
}
