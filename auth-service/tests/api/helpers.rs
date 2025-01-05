use auth_service::Application;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
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
    pub async fn get_route(&self, route: &str) -> reqwest::Response {
        dbg!(&self.address);
        self.http_client
            .get(&format!("{}{}", &self.address, &route))
            .send()
            .await
            .expect(format!("Familed to execute request to route: {:?}", route).as_str())
    }
    pub async fn post_route(&self, route: &str) -> reqwest::Response {
        dbg!(&self.address);
        self.http_client
            .post(&format!("{}{}", &self.address, &route))
            .send()
            .await
            .expect(format!("Familed to execute request to route: {:?}", route).as_str())
    }
}
