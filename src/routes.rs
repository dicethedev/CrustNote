use axum::{routing::{get, post, delete}, Router};

use crate::handlers::{list_tasks, create_task, delete_task, Db};

pub fn create_router(db: Db) -> Router {
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", delete(delete_task))
        .with_state(db)
        .route("/health", get(|| async { "OK" }))
}
