use crate::repository::{Repository, User, UserToCreate};
use axum::http::StatusCode;
use axum::routing::{delete, get};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json, Router,
};

pub fn routes<T: Repository + Clone + Send + Sync + 'static>() -> Router<T> {
    Router::new()
        .route("/users", get(get_users::<T>).post(add_user::<T>))
        .route("/users/:id", delete(remove_user::<T>))
}

pub async fn get_users<T: Repository>(State(repo): State<T>) -> impl IntoResponse {
    Json(repo.get_users().await)
}

pub async fn remove_user<T: Repository>(
    State(mut repo): State<T>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match repo.remove_user(id).await {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(user) => Json(user).into_response(),
    }
}

pub async fn add_user<T: Repository>(
    State(mut repo): State<T>,
    Json(user): Json<UserToCreate>,
) -> impl IntoResponse {
    let user = repo.add_user(&user).await;
    Json(user)
}
