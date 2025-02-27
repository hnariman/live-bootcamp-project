use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, BannedTokenStore},
    services::HashsetBannedTokenStore,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(_state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;
    let token = cookie.value();

    let mut banned_tokens: HashsetBannedTokenStore = _state.banned_tokens.read().await.clone();

    if validate_token(&token, banned_tokens.clone()).await.is_err() {
        return Err(AuthAPIError::InvalidToken);
    }

    let _ = banned_tokens.add(token.to_string()).await;

    let jar = jar.remove(JWT_COOKIE_NAME);
    Ok((jar, StatusCode::OK))
}
