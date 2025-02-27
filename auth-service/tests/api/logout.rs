use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::{BannedTokenError, BannedTokenStore, User},
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
};
use reqwest::Url;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    // adjust
    let app = TestApp::new().await;
    let user = User::new(&get_random_email(), "!@#(*$&#!234234alsdkj!@#", false).unwrap();
    let test_case = serde_json::json!({ "email": user.email.as_ref()});

    // act
    let response = app.post_logout(&test_case).await;

    // assert
    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Coult not deserialize response body to Error Response")
            .error,
        "Missing token".to_owned()
    )
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let user = User::new(&get_random_email(), "!@#(*$&#!234234alsdkj!@#", false).unwrap();

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let test_case = serde_json::json!({ "email": user.email.as_ref()});
    //act
    let response = app.post_logout(&test_case).await;
    //assert
    assert_eq!(response.status().as_u16(), 401);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Coult not deserialize response body to Error Response")
            .error,
        "Invalid token".to_owned()
    )
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;
    let user = User::new(&get_random_email(), "!@#(*$&#!234234alsdkj!@#", false).unwrap();

    signup(&app, &user).await;

    let response = login(&app, &user).await;

    let token = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    eprintln!("===Cookies=== \n {:?}", token.value());

    assert!(!token.value().is_empty());

    let test_case = serde_json::json!({ "email": user.email.as_ref()});
    let response = app.post_logout(&test_case).await;

    let banned = app.banned_tokens.read().await;
    assert_eq!(
        banned.check(token.value().to_string()).await,
        Err(BannedTokenError::BannedToken)
    );
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;
    let user = User::new(&get_random_email(), "!@#(*$&#!234234alsdkj!@#", false).unwrap();
    signup(&app, &user).await;
    let _ = login(&app, &user).await;
    let test_case = serde_json::json!({ "email": user.email.as_ref()});
    let response1 = app.post_logout(&test_case).await;
    let response2 = app.post_logout(&test_case).await;
    assert_eq!(response1.status().as_u16(), 200);
    assert_eq!(response2.status().as_u16(), 400);
}
//TODO : shall I move this to helpers for cleaner codebase?
//or just follow grasp and leave responsibility to the closest context?
async fn signup(app: &TestApp, user: &User) {
    eprintln!("==================================================== signup attempt");
    let signup_body = serde_json::json!({
        "email": user.email.as_ref(),
        "password": user.password.as_ref(),
        "requires2FA": user.requires_2fa
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
}

async fn login(app: &TestApp, user: &User) -> reqwest::Response {
    eprintln!("==================================================== login attempt");
    let login_body = serde_json::json!({
        "email": user.email.as_ref(),
        "password": user.password.as_ref()
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);
    response
}
