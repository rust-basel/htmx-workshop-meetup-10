use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct UserToCreate {
    pub name: String,
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[async_trait]
pub trait Repository {
    async fn add_user(&mut self, user: &UserToCreate) -> User;
    async fn remove_user(&mut self, id: i64) -> Option<User>;
    async fn get_users(&self) -> Vec<User>;
}
