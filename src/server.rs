use std::env;

use axum::{routing::get, Router};
use colored::Colorize;
use tokio::net::TcpListener;

use crate::{qr_code, technical_endpoints::healthz};

pub async fn create_server() -> anyhow::Result<(TcpListener, Router)> {
    let port: String = env::var("SERVER_PORT").unwrap_or("3000".to_string());
    let binding = format!("0.0.0.0:{}", port);
    println!("listeng at: {}", binding.green());

    // build our application with a single route
    let app = Router::new().nest("/", make_api());

    let listener = tokio::net::TcpListener::bind(binding).await.unwrap();

    Ok((listener, app))
}

fn make_api() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(qr_code::qr_code_html))
        .route("/qrcodes", get(qr_code::qr_code_as_picture))
}
