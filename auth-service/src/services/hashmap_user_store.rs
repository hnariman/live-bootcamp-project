#![warn(clippy::all, clippy::pedantic)]

use crate::domain::{Email, Password, User, UserStore, UserStoreError};
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, Mutex},
};

#[derive(Debug, Default, Clone)]
pub struct HashmapUserStore {
    pub users: Arc<Mutex<HashMap<Email, User>>>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // awesome ideomatic idea! make sure to remember and reuse it often!
        match self.users.lock().unwrap().entry(user.email.clone()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            }
        }
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        let email = Email::parse(email)?;
        match self.users.lock().unwrap().get(&email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &'static str,
        password: &str,
    ) -> Result<(), UserStoreError> {
        let email = Email::parse(email)?;
        let _password = Password::parse(password)?;

        match self.users.lock().unwrap().entry(email.clone()) {
            Entry::Occupied(u) => {
                if _password != u.get().password {
                    Err(UserStoreError::InvalidCredentials)
                } else {
                    Ok(())
                }
            }
            Entry::Vacant(_) => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::CreateUserError;

    use super::*;

    #[tokio::test]
    pub async fn test_add_user() {
        let mut storage = HashmapUserStore::default();
        let mock = User::new("hnariman@gmail.com", "123oi1u23", false).unwrap();
        let mock2 = User::new("h.nariman@gmail.com", "123oi1u23", false).unwrap();
        let _added_mock = storage.add_user(mock).await;
        let _added_mock2 = storage.add_user(mock2).await;

        assert_eq!(storage.users.lock().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_add_user_existing_user() {
        let mut storage = HashmapUserStore::default();
        let mock = User::new("h.nariman@gmail.com", "123oi1u23", false).unwrap();
        let mock2 = User::new("h.nariman@gmail.com", "123oi1u23", false).unwrap();
        let _added_mock = storage.add_user(mock).await;
        let expected = storage.add_user(mock2).await;

        assert_eq!(expected, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    pub async fn test_add_user_short_password() {
        let expected = User::new("h.nariman@gmail.com", "123", false).map_err(|e| e);
        assert_eq!(expected, Err(CreateUserError::InvalidPassword));
    }

    #[tokio::test]
    pub async fn test_add_user_invalid_email() {
        let expected = User::new("h.narimangmail.com", "123", false).map_err(|e| e);
        assert_eq!(expected, Err(CreateUserError::InvalidEmail));
    }

    #[tokio::test]
    pub async fn test_get_user() {
        let mut storage = HashmapUserStore::default();
        let mock = User::new("tnariman@gmail.com", "123oi1u23", false).unwrap();
        let _added_mock = storage.add_user(mock.clone()).await;

        let found = storage.get_user(&mock.email.as_str()).await.unwrap();

        assert_eq!(found, mock);
    }

    #[tokio::test]
    pub async fn test_validate_user_shall_throw_invalid_credentials() {
        let storage = HashmapUserStore::default();
        let mock = User::new("hnariman@gmail.com", "123asdf987234", false).unwrap();

        storage
            .users
            .lock()
            .unwrap()
            .insert(mock.email.clone(), mock);

        let validation_result = storage
            .validate_user("hnariman@gmail.com", "123asdf98723")
            .await;

        let expected = Err(UserStoreError::InvalidCredentials);

        assert_eq!(validation_result, expected);
    }

    #[tokio::test]
    pub async fn test_validate_user_shall_throw_user_not_found_wrong_email() {
        let email = "testing@gmail.com";
        let pass = "123asldkfj123";
        let storage = HashmapUserStore::default();

        let mock = User::new(email, pass, false).expect("unable to create mock user for test");

        storage
            .users
            .lock()
            .unwrap()
            .insert(mock.email.clone(), mock);

        let validation_result = storage.validate_user("testingssss@gmail.com", pass).await;
        let expected = Err(UserStoreError::UserNotFound);

        assert_eq!(validation_result, expected);
    }
}
