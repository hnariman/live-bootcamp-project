use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{domain::AuthAPIError, utils::auth::validate_token};

#[derive(serde::Deserialize, Debug)]
pub struct VerifyTokenRequest {
    token: String,
}

pub async fn verify_token(
    Json(_request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = _request.token;

    if token.is_empty() {
        return Err(AuthAPIError::MalformedToken);
    }

    if validate_token(&token).await.is_err() {
        return Err(AuthAPIError::InvalidToken);
    }

    Ok(StatusCode::OK.into_response())
}
