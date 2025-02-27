#![warn(clippy::all, clippy::pedantic)]

use crate::domain::{BannedTokenError, BannedTokenStore};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

#[derive(Debug, Default, Clone)]
pub struct HashsetBannedTokenStore {
    pub banned: Arc<Mutex<HashSet<String>>>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add(&mut self, _data: String) -> Result<(), BannedTokenError> {
        let mut banned = self.banned.lock().map_err(|_| BannedTokenError::Poisoned)?;
        banned.insert(_data);
        Ok(())
    }

    async fn check(&self, _data: String) -> Result<(), BannedTokenError> {
        let data = self.banned.lock().map_err(|_| BannedTokenError::Poisoned)?;

        if data.get(_data.as_str()).is_some() {
            return Err(BannedTokenError::BannedToken);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::services::HashsetBannedTokenStore;

    #[tokio::test]
    pub async fn test_add_token() {
        let mut storage = HashsetBannedTokenStore::default();

        let _ = storage.add(String::from("asldkfjasl;dkj")).await.unwrap();
        let _ = storage.add(String::from("woeiruowieulas")).await.unwrap();

        assert_eq!(storage.banned.lock().unwrap().len(), 2);
    }

    #[tokio::test]
    pub async fn test_add_check() {
        let mut storage = HashsetBannedTokenStore::default();
        let token = String::from("asldkfjalsdkjf");

        let _ = storage.add(token.clone()).await.unwrap();

        assert_eq!(
            storage.check(token).await,
            Err(BannedTokenError::BannedToken)
        )
    }
}
