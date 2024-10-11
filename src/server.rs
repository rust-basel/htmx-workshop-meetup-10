use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use colored::Colorize;
use tokio::net::TcpListener;

use crate::qr_code;
use crate::{assets, qr_code::persictence::QrCodeInMemoryDb, technical_endpoints::healthz};

pub async fn create_server() -> anyhow::Result<(TcpListener, Router)> {
    let port: String = env::var("SERVER_PORT").unwrap_or("3000".to_string());
    let binding = format!("0.0.0.0:{}", port);
    println!("listening at: {}", binding.green());

    let db = QrCodeInMemoryDb::new();
    let (id, code) = qr_code::endpoints::create_image().await;
    db.set(id.clone(), code).await;

    let app = Router::new()
        .nest("/", make_api())
        .with_state(db)
        .nest("/", assets());

    let listener = tokio::net::TcpListener::bind(binding).await.unwrap();

    Ok((listener, app))
}

fn assets() -> Router {
    Router::new().nest_service("/_assets", assets::using_serve_dir())
}

fn make_api() -> Router<QrCodeInMemoryDb> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(qr_code::endpoints::page))
        .route("/qrcodes", get(qr_code::endpoints::qr_code_as_picture))
        .route("/qrcodes", post(qr_code::endpoints::create))
        .route("/qr_table", get(qr_code::endpoints::qr_table))
}
