use server::create_server;

pub mod assets;
pub mod qr_code;
pub mod server;
pub mod technical_endpoints;

#[tokio::main]
async fn main() {
    start().await
}

pub async fn start() {
    let (listener, app) = create_server().await.expect("could not create server");
    axum::serve(listener, app)
        .await
        .expect("could not start server");
}
