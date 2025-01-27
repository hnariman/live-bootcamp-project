use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

fn bad_request() -> (StatusCode, Json<SignupResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(SignupResponse {
            message: "Bad request".to_string(),
        }),
    )
}
#[axum::debug_handler]
pub async fn signup(
    State(state): State<AppState>,
    Json(_request): Json<SignupRequest>,
) -> impl IntoResponse {
    let user = match User::new(&_request.email, &_request.password) {
        Ok(user) => user,
        Err(_) => return bad_request(),
    };

    let mut user_store = state.user_store.write().await;

    match user_store.add_user(user) {
        Err(_) => return bad_request(),
        Ok(_) => print!("allok"),
    }

    // let data = state.user_store.read().await;
    // dbg!(&data);
    // drop(data);

    // if user_store.add_user(user).is_err() {
    //     return bad_request();
    // }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}
