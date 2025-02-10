// #[derive(Debug, thiserror::Error)]
pub enum AuthAPIError {
    // #[error("user already exists")]
    UserAlreadyExists,
    // #[error("invalid user credentials")]
    InvalidUserCredentials,
    // #[error("unexpected error")]
    UnexpectedError,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CreateUserError {
    #[error("Invalid user")]
    InvalidPassword,
    #[error("Invalid email")]
    InvalidEmail,
}
