use axum::{response::Html, routing::get, serve::Serve, Router};
use tower_http::services::ServeDir;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/hello", get(hello_handler));

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

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Mission Complete!</h1>")
}
