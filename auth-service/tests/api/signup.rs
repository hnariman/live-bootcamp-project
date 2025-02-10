use auth_service::ErrorResponse;

#[allow(unused)]
use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn signup_should_return_422_if_malformed() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({ "password":"password123", "requires2FA":true }),
        serde_json::json!({ "password":"pas", "requires2FA":true }),
        serde_json::json!({ "password":"sword", "requires2FA":true }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        )
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let test_cases = [serde_json::json!(
        { "email":"fast@gmail.com", "password":"123lkjslk##dfj@@laskdjf", "requires2FA":false }
    )];

    for each in test_cases.iter() {
        let response = app.post_signup(each).await;
        assert_eq!(response.status().as_u16(), 201, "Failed: {:?}", each);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [serde_json::json!(
        { "email":"asdfasdf_not_an_email", "password":"123lkjslkdfjlaskdjf", "requires2FA":false })];

    for each in test_cases.iter() {
        let response = app.post_signup(each).await;
        assert_eq!(response.status().as_u16(), 400, "Failed: {:?}", each);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Coult not deserialize response body to Error Response")
                .error,
            "Invalid credentials".to_owned()
        )
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let test_cases = [serde_json::json!( {
        "email":"existing@user.com",
        "password":"1234!@#$@!#$(&*&*qwer1234",
        "requires2FA":true
    })];

    for each in test_cases.iter() {
        let response = app.post_signup(each).await;
        assert_eq!(response.status().as_u16(), 409);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Coult not deserialize response body to Error Response")
                .error,
            "User already exists".to_owned()
        )
    }
}
