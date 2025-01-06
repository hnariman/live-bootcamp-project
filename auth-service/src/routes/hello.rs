use axum::response::Html;

pub async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Mission Complete!</h1>")
}
