use std::sync::Arc;

use auth_service::{
    app_state::AppState,
    services::{HashmapUserStore, HashsetBannedTokenStore},
    utils::constants::prod,
    Application,
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_tokens = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let app_state = AppState::new(user_store, banned_tokens);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build App");

    app.run().await.expect("failed to run app");
}
