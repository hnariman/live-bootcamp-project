use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;
    let token = cookie.value();

    if validate_token(&token).await.is_err() {
        return Err(AuthAPIError::InvalidToken);
    }

    let jar = jar.remove(JWT_COOKIE_NAME);
    Ok((jar, StatusCode::OK))
}
