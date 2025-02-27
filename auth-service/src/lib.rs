use app_state::AppState;
use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(
        app_state: AppState,
        address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://[droplet_api]:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let _response_200 = || async { StatusCode::OK.into_response() };
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-token", post(routes::verify_token))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/hello", get(routes::hello_handler))
            .with_state(app_state)
            .layer(cors);

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
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"), // 409
            AuthAPIError::InvalidUserCredentials => {
                (StatusCode::BAD_REQUEST, "Invalid credentials") // 400
            }
            AuthAPIError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"), // 401

            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Invalid credentials") // 500
            }
            AuthAPIError::UserNotFound => (StatusCode::UNPROCESSABLE_ENTITY, "bas request"), // 422
            AuthAPIError::InvalidCredentials => {
                (StatusCode::UNPROCESSABLE_ENTITY, "invalid user credentials")
            } // 422
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),       // 401
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token"),        // 400
            AuthAPIError::MalformedToken => (StatusCode::UNPROCESSABLE_ENTITY, "Malformed token"), // 422
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
