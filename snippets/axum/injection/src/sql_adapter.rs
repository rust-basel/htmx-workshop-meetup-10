use axum::async_trait;
use sqlx::SqlitePool;

use crate::repository::{Repository, User, UserToCreate};

#[derive(Clone)]
pub struct SqliteAdapter {
    pool: SqlitePool,
}

impl SqliteAdapter {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for SqliteAdapter {
    async fn add_user(&mut self, user: UserToCreate) -> User {
        todo!()
    }
    async fn remove_user(&mut self, id: i64) -> Option<User> {
        todo!()
    }
    async fn get_users(&self) -> Vec<User> {
        todo!()
    }
}
