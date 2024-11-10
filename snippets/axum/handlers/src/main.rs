use axum::{response::IntoResponse, routing::get, Router};

async fn hello_world_handler() -> impl IntoResponse {
    "Hello World"
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello_world_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
