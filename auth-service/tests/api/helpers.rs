use std::sync::Arc;

use auth_service::{
    app_state::{AppState, UserStoreType},
    domain::{Email, Password, User},
    services::HashmapUserStore,
    utils::constants::test,
    Application,
};

use auth_service::domain::UserStore;
use reqwest::cookie::Jar;
use tokio::sync::RwLock;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let mut mock_store = HashmapUserStore::default();

        let _existing_user = User::new(
            Email::parse("existing@user.com").unwrap().as_ref(),
            Password::parse("!@#(*$&#!234234alsdkj!@#")
                .unwrap()
                .as_ref(),
            true,
        )
        .expect("unable to created existing user for tests");

        mock_store
            .add_user(_existing_user)
            .await
            .expect("unable to add mock user");

        let user_store: UserStoreType = Arc::new(RwLock::new(mock_store));
        let mock_state = AppState::new(user_store);

        let app = Application::build(mock_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        TestApp {
            address,
            cookie_jar,
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
        dbg!(&self.address);
        self.http_client
            .post(&format!("{}{}", &self.address, &route))
            .send()
            .await
            .expect(format!("Familed to execute request to route: {:?}", route).as_str())
    }

    pub async fn post_logout<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute post logout request")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute post login request")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", uuid::Uuid::new_v4())
}
