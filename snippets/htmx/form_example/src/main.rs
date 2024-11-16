use axum::response::IntoResponse;
use axum::routing::post;
use axum::Form;
use serde::Deserialize;
use std::env;
use tower_http::services::ServeDir;

use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    start().await;
}

pub async fn start() {
    let (listener, app) = create_server().await;
    axum::serve(listener, app).await.unwrap();
}

pub async fn create_server() -> (TcpListener, Router) {
    let port: String = env::var("SERVER_PORT").unwrap_or("3000".to_string());
    let binding = format!("0.0.0.0:{}", port);
    println!("listening at: {}", binding);

    let app = Router::new().nest("/", assets()).nest("/", endpoints());

    let listener = tokio::net::TcpListener::bind(binding).await.unwrap();

    (listener, app)
}

fn assets() -> Router {
    Router::new().nest_service("/", using_serve_dir())
}

fn endpoints() -> Router {
    Router::new().route("/message", post(message))
}

#[derive(Deserialize)]
pub struct MessageForm {
    message: String,
}

async fn message(Form(form): Form<MessageForm>) -> impl IntoResponse {
    format!("Message: {}", form.message)
}

pub fn using_serve_dir() -> ServeDir {
    ServeDir::new("assets")
}
