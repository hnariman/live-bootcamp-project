use std::sync::Arc;
use tokio::sync::RwLock;

use crate::services::{hashmap_user_store::HashmapUserStore, HashsetBannedTokenStore};

pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;
pub type BannedTokensType = Arc<RwLock<HashsetBannedTokenStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_tokens: BannedTokensType,
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_tokens: BannedTokensType) -> Self {
        Self {
            user_store,
            banned_tokens,
        }
    }
}
