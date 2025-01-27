use std::sync::Arc;

use auth_service::{app_state::AppState, services::HashmapUserStore, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build App");

    app.run().await.expect("failed to run app");
}
