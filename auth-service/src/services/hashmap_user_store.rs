#![warn(clippy::all, clippy::pedantic)]

use crate::domain::{CreateUserError, Email, Password, User};
use std::collections::HashMap;

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

#[derive(Debug, Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.entry(user.email.clone()).or_insert(user.clone());

        if !self.users.contains_key(&user.email) {
            return Err(UserStoreError::UnableToCreateUser);
        }

        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        let email = Email::from(email)?;
        let result = self.users.get(&email).unwrap();
        Ok(result.clone())
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let email = Email::from(email)?;
        let password = Password::from(password)?;
        let user = self.users.get(&email).ok_or(UserStoreError::UserNotFound)?;

        if user.password.as_str() != password.as_str() {
            return Err(UserStoreError::InvalidCredentials);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_add_user() {
        let mut storage = HashmapUserStore::default();
        let mock = User::new("hnariman@gmail.com", "123oi1u23").unwrap();
        let mock2 = User::new("h.nariman@gmail.com", "123oi1u23").unwrap();
        let _ = storage.add_user(mock);
        let _ = storage.add_user(mock2);
        assert_eq!(storage.users.len(), 2);
    }

    #[tokio::test]
    pub async fn test_add_user_existing_user() {
        let mut storage = HashmapUserStore::default();
        let mock = User::new("h.nariman@gmail.com", "123oi1u23").unwrap();
        let mock2 = User::new("h.nariman@gmail.com", "123oi1u23").unwrap();
        let _ = storage.add_user(mock);
        let expected = storage.add_user(mock2).map_err(|e| e);

        assert_eq!(expected, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    pub async fn test_add_user_short_password() {
        let expected = User::new("h.nariman@gmail.com", "123").map_err(|e| e);
        assert_eq!(expected, Err(CreateUserError::InvalidPassword));
    }

    #[tokio::test]
    pub async fn test_add_user_invalid_email() {
        let expected = User::new("h.narimangmail.com", "123").map_err(|e| e);
        assert_eq!(expected, Err(CreateUserError::InvalidEmail));
    }

    #[tokio::test]
    pub async fn test_get_user() {
        let mut storage = HashmapUserStore::default();
        let mock = User::new("tnariman@gmail.com", "123oi1u23").unwrap();

        storage.add_user(mock.clone()).ok();

        let found = storage.get_user(&mock.email.as_str()).unwrap();

        assert_eq!(found, mock);
    }

    #[tokio::test]
    pub async fn test_validate_user_shall_throw_invalid_credentials() {
        let mut storage = HashmapUserStore::default();
        let mock = User::new("hnariman@gmail.com", "123asdf987234").unwrap();
        storage.users.insert(mock.email.clone(), mock);

        let validation_result = storage
            .validate_user("hnariman@gmail.com", "123asdf98723")
            .map_err(|e| e);
        let expected = Err(UserStoreError::InvalidCredentials);

        assert_eq!(validation_result, expected);
    }

    #[tokio::test]
    pub async fn test_validate_user_shall_throw_user_not_found_wrong_email() {
        let mut storage = HashmapUserStore::default();
        let mock = User::new("hnariman@gmail.com", "123asdf987234").unwrap();
        storage.users.insert(mock.email.clone(), mock);

        let validation_result = storage
            .validate_user("nariman@gmail.com", "123asdf987234")
            .map_err(|e| e);
        let expected = Err(UserStoreError::UserNotFound);

        assert_eq!(validation_result, expected);
    }
}
