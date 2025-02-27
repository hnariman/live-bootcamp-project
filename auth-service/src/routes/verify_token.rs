use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app_state::AppState, domain::AuthAPIError, services::HashsetBannedTokenStore,
    utils::auth::validate_token,
};

#[derive(serde::Deserialize, Debug)]
pub struct VerifyTokenRequest {
    token: String,
}

pub async fn verify_token(
    State(_state): State<AppState>,
    Json(_request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = _request.token;

    let banned: HashsetBannedTokenStore = _state.banned_tokens.read().await.clone();

    if token.is_empty() {
        return Err(AuthAPIError::MalformedToken);
    }

    if validate_token(&token, banned).await.is_err() {
        return Err(AuthAPIError::InvalidToken);
    }

    Ok(StatusCode::OK.into_response())
}
