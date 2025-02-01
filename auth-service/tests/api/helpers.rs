use std::sync::Arc;

use auth_service::{
    app_state::{AppState, UserStoreType},
    domain::{Email, Password, User},
    services::HashmapUserStore,
    Application,
};
use tokio::sync::RwLock;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let mut mock_store = HashmapUserStore::default();

        let _existing_user = User::new(
            Email::from("existing@user.com").unwrap().as_str(),
            Password::from("1234qwer1234").unwrap().as_str(),
            true,
        )
        .unwrap();
        mock_store.add_user(_existing_user).unwrap();

        let user_store: UserStoreType = Arc::new(RwLock::new(mock_store));
        let mock_state = AppState::new(user_store);

        let app = Application::build(mock_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());
        let http_client = reqwest::Client::new();
        TestApp {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }
    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_route(&self, route: &str) -> reqwest::Response {
        self.http_client
            .post(&format!("{}{}", &self.address, &route))
            .send()
            .await
            .expect(format!("Familed to execute request to route: {:?}", route).as_str())
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", uuid::Uuid::new_v4())
}
