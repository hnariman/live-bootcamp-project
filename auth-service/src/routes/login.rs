use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, UserStore},
    // domain::{AuthAPIError, CreateUserError, Email, Password, User, UserStore, UserStoreError},
    utils::auth::generate_auth_cookie,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
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

// #[axum::debug_handler]
pub async fn login(
    State(_state): State<AppState>,
    jar: CookieJar,
    Json(_request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = _request.email;
    let password = _request.password;

    let malformed = email.is_empty() || password.is_empty();

    if malformed {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let email = Email::parse(&email).map_err(|e| {
        eprint!("email error: {:?}", e);
        AuthAPIError::InvalidCredentials
    })?;
    let password = Password::parse(&password).map_err(|_| AuthAPIError::InvalidUserCredentials)?;

    let db = _state.user_store.read().await;

    if db
        .validate_user(email.as_ref(), password.as_ref())
        .await
        .is_err()
    {
        return Err(AuthAPIError::Unauthorized);
    };

    let user = db
        .get_user(email.as_ref())
        .await
        .map_err(|_| AuthAPIError::Unauthorized)?;

    let auth_cookie =
        generate_auth_cookie(&user.email).map_err(|_| AuthAPIError::UnexpectedError)?;

    let authorized = &jar.add(auth_cookie);

    Ok((authorized.clone(), StatusCode::OK.into_response()))
}
