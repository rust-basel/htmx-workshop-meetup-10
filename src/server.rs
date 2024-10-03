use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use colored::Colorize;
use tokio::net::TcpListener;

use crate::{
    qr_code::{
        persictence::QrCodeInMemoryDb,
        qr_code_endpoints::{
            create_a_pr_code_image, create_qr_code, qr_code_as_picture, qr_code_html,
        },
    },
    technical_endpoints::healthz,
};

pub async fn create_server() -> anyhow::Result<(TcpListener, Router)> {
    let port: String = env::var("SERVER_PORT").unwrap_or("3000".to_string());
    let binding = format!("0.0.0.0:{}", port);
    println!("listeng at: {}", binding.green());

    let db = QrCodeInMemoryDb::new();
    let (id, code) = create_a_pr_code_image().await;
    db.set(id.clone(), code).await;

    let app = Router::new().nest("/", make_api()).with_state(db);

    let listener = tokio::net::TcpListener::bind(binding).await.unwrap();

    Ok((listener, app))
}

fn make_api() -> Router<QrCodeInMemoryDb> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(qr_code_html))
        .route("/qrcodes", get(qr_code_as_picture))
        .route("/qrcodes", post(create_qr_code))
}
