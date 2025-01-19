#![warn(clippy::all, clippy::pedantic)]

use crate::domain::{Email, Password, User};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Something went wrong")]
    UnexpectedError,
}

#[derive(Debug, Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: &User) -> Result<(), UserStoreError> {
        if let = Ok(self.users.contains_key(&user.email)) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user.clone());

        Ok(())
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        let email = Email::from(email).expect("unable to create email");

        match self.users.get(&email) {
            None => Err(UserStoreError::UserNotFound),
            Some(user) => Ok(user.clone()),
        }
    }

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let email = Email::from(email).unwrap();
        let password = Password::from(password).unwrap();

        let user = if let Some(found_user) = self.users.get(&email) {
            found_user
        } else {
            return Err(UserStoreError::UserNotFound);
        };

        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }

        Ok(())
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_add_user() {
        let mut storage = HashmapUserStore::default();
        let newbie = User::new("hnariman@gmail.com", "123oi1u23").expect("unable to create user");
        let _ = storage.add_user(&newbie);
        assert_eq!(storage.users.len(), 1);
    }

    #[tokio::test]
    pub async fn test_get_user() {
        let storage = HashmapUserStore::default();
        let newbie = User::new("hnariman@gmail.com", "123oi1u23").expect("unable to create user");
        let _ = storage.add_user(&newbie);

        let user = storage.get_user(newbie.email.clone()).unwrap();
        assert_eq!(user, newbie);
    }

    // #[tokio::test]
    // pub async fn test_validate_user() {
    //     todo!()
    // }
}
