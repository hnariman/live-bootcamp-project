#[derive(Debug, thiserror::Error)]
pub enum AuthAPIError {
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("invalid user credentials")]
    InvalidUserCredentials,
    #[error("unexpected error")]
    InvalidCredentials,
    #[error("invalid credentials")]
    UnexpectedError,
    #[error("user not found")]
    UserNotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("missing token")]
    MissingToken,
    #[error("invalid token")]
    InvalidToken,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CreateUserError {
    #[error("Invalid user")]
    InvalidPassword,
    #[error("Invalid email")]
    InvalidEmail,
}
