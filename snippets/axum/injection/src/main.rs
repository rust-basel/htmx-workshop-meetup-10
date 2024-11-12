mod repository;
mod sql_adapter;
mod user;

use crate::sql_adapter::SqliteAdapter;
use axum::Router;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let adapter = SqliteAdapter::new(pool);
    let router = Router::new().merge(user::routes()).with_state(adapter);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
