use crate::{
    app_state::AppState,
    domain::{AuthAPIError, CreateUserError, Email, UserStore, UserStoreError},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub email: String,
}

// It's a good place to handle all propagated errors
// since we have the most context here
pub async fn login(
    State(_state): State<AppState>,
    Json(_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let store = _state.user_store.read().await;
    // If JSON is missing or malformed -> 422
    // If JSON contains invalid credentials -> 400
    // If JSON contains credentials that are valid but incorrect -> 401

    dbg!(&_request);
    let _email = Email::parse(&_request.email).map_err(|_| AuthAPIError::InvalidUserCredentials);
    let _passw = Email::parse(&_request.password).map_err(|_| AuthAPIError::InvalidUserCredentials);

    match store.get_user(&_request.email).await {
        Ok(data) => Ok((
            StatusCode::OK,
            Json(LoginResponse {
                email: String::from(data.email.as_str()),
            }),
        )),
        Err(UserStoreError::UserNotFound) => Err(AuthAPIError::UserNotFound),
        Err(UserStoreError::UnexpectedError(creds_error)) => match creds_error {
            CreateUserError::InvalidEmail => Err(AuthAPIError::Unauthorized),
            CreateUserError::InvalidPassword => Err(AuthAPIError::Unauthorized),
        },
        _ => Err(AuthAPIError::UnexpectedError),
    }
}
