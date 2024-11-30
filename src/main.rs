use server::create_server;

pub mod assets;
pub mod qr_code;
pub mod server;
pub mod technical_endpoints;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let (_, app) = start().await;
    Ok(app.into())
}

pub async fn start() -> (tokio::net::TcpListener, axum::Router) {
    create_server().await.expect("could not create server")
}
