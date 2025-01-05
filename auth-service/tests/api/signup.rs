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
