use axum::async_trait;
use sqlx::SqlitePool;

use crate::repository::{Repository, User, UserToCreate};

#[derive(Clone)]
pub struct SqliteAdapter {
    pool: SqlitePool,
}

impl SqliteAdapter {
    pub async fn new(pool: SqlitePool) -> Self {
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
        Self { pool }
    }
}

#[async_trait]
impl Repository for SqliteAdapter {
    async fn add_user(&mut self, user: &UserToCreate) -> User {
        todo!()
    }
    async fn remove_user(&mut self, id: i64) -> Option<User> {
        todo!()
    }
    async fn get_users(&self) -> Vec<User> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::{Repository, UserToCreate};
    use crate::sql_adapter::SqliteAdapter;
    use sqlx::SqlitePool;

    #[sqlx::test]
    async fn given_user_to_add_when_add_user_then_return_created_user(pool: SqlitePool) {
        // given
        let mut adapter = SqliteAdapter::new(pool).await;
        let user = UserToCreate {
            name: "Hans".to_string(),
        };

        // when
        let added_user = adapter.add_user(&user).await;

        // then
        assert_eq!(added_user.name, user.name);
    }

    #[sqlx::test]
    async fn when_get_users_then_return_users(pool: SqlitePool) {
        // given
        let adapter = SqliteAdapter::new(pool).await;

        // when
        let users = adapter.get_users().await;

        // then
        assert_eq!(users.len(), 0);
    }

    #[sqlx::test(fixtures("existing_users"))]
    async fn given_seeded_users_when_get_users_then_return_users(pool: SqlitePool) {
        // given
        let adapter = SqliteAdapter::new(pool).await;

        // when
        let users = adapter.get_users().await;

        // then
        assert_eq!(users.len(), 3);
    }

    #[sqlx::test()]
    async fn given_no_user_when_remove_user_then_return_none(pool: SqlitePool) {
        // given
        let mut adapter = SqliteAdapter::new(pool).await;

        // when
        let deleted_user = adapter.remove_user(1).await;

        // then
        assert!(deleted_user.is_none());
    }

    #[sqlx::test(fixtures("existing_users"))]
    async fn given_seeded_users_when_remove_user_then_return_deleted_user(pool: SqlitePool) {
        // given
        let mut adapter = SqliteAdapter::new(pool).await;

        // when
        let deleted_user = adapter.remove_user(1).await;

        // then
        assert!(deleted_user.is_some());
        assert_eq!(deleted_user.unwrap().name, "Alice");
    }
}
