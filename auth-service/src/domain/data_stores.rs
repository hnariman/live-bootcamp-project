use super::{CreateUserError, User};

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, _user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, _email: &str) -> Result<User, UserStoreError>;
    async fn validate_user(&self, _email: &str, _password: &str) -> Result<(), UserStoreError>;
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Something went wrong")]
    UnexpectedError(#[from] CreateUserError),
    #[error("invalid user")]
    UnableToCreateUser,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum BannedTokenError {
    #[error("Something went wrong")]
    UnexpectedError,
    #[error("Token is banned")]
    BannedToken,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Mutex lock poisoned")]
    Poisoned,
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add(&mut self, _data: String) -> Result<(), BannedTokenError>;
    async fn check(&self, _data: String) -> Result<(), BannedTokenError>;
}
