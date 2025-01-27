use app_state::AppState;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    serve::Serve,
    Router,
};
use tower_http::services::ServeDir;

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(
        app_state: AppState,
        address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let _response_200 = || async { StatusCode::OK.into_response() };
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-token", post(routes::verify_token))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/hello", get(routes::hello_handler))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;

        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);
        let app = Application { server, address };
        Ok(app)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("server is listening on {}", &self.address);
        self.server.await
    }
}
