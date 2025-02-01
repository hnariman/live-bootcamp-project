use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User},
    services::UserStoreError,
};

#[axum::debug_handler]
pub async fn signup(
    State(state): State<AppState>,
    Json(_request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::from(&_request.email).map_err(|_| AuthAPIError::InvalidUserCredentials)?;

    let password =
        Password::from(&_request.password).map_err(|_| AuthAPIError::InvalidUserCredentials)?;

    let user = User::new(email.as_str(), password.as_str(), _request.requires_2fa);
    let user = user.map_err(|_| AuthAPIError::UnexpectedError)?;

    // we don't unlock mutex unless validation is ok
    let mut user_store = state.user_store.write().await;

    user_store.add_user(user).map_err(|e| match e {
        UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
        _ => AuthAPIError::UnexpectedError,
    })?;

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize, Debug)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Debug)]
pub struct SignupResponse {
    pub message: String,
}
