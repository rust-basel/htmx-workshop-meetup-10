use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::qr_code::{self};
use crate::{assets, qr_code::persictence::QrCodeInMemoryDb, technical_endpoints::healthz};

pub async fn create_server() -> anyhow::Result<(TcpListener, Router)> {
    let port: String = env::var("SERVER_PORT").unwrap_or("3000".to_string());
    let binding = format!("0.0.0.0:{}", port);
    println!("listening at: {}", binding);

    let db = QrCodeInMemoryDb::new();

    let app = Router::new()
        .nest("/", make_api())
        .with_state(db)
        .nest("/", assets());

    let listener = TcpListener::bind(binding).await?;

    Ok((listener, app))
}

fn assets() -> Router {
    Router::new()
        .route("/_assets/htmx.min.js", get(assets::include_htmx))
        .route("/_assets/app.css", get(assets::include_app_css))
        .route("/_assets/mvp.css", get(assets::include_mvp_css))
}

fn make_api() -> Router<QrCodeInMemoryDb> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(qr_code::endpoints::page))
        // qr code endpoints
        .route("/qrcodes", get(qr_code::endpoints::image))
        .route("/qrcodes", post(qr_code::endpoints::create))
        .route("/debug", get(qr_code::endpoints::debug))
        // game endpoints
        .route("/game", get(qr_code::endpoints::game_view))
        .route("/game/inc", post(qr_code::endpoints::inc))
        .route("/game/dec", post(qr_code::endpoints::dec))
        .route("/game/current", get(qr_code::endpoints::current))
}
