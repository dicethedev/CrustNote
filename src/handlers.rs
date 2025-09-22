use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::task::Task;

pub type Db = Arc<Mutex<Vec<Task>>>;

pub async fn list_tasks(State(db): State<Db>) -> Json<Vec<Task>> {
    let tasks = db.lock().unwrap();
    Json(tasks.clone())
}

pub async fn create_task(
    State(db): State<Db>, 
    Json(payload): Json<serde_json::Value>
) -> Json<Task> {
    let title = payload["title"].as_str().unwrap_or("Untitled").to_string();
    let task = Task::new(title);

    db.lock().unwrap().push(task.clone());
    Json(task)
}

pub async fn delete_task(
    State(db): State<Db>, 
    Path(id): Path<String>
) -> Json<serde_json::Value> {
    let mut tasks = db.lock().unwrap();
    let before = tasks.len();
    tasks.retain(|t| t.id.to_string() != id);

    if tasks.len() < before {
        Json(json!({ "status": "deleted" }))
    } else {
        Json(json!({ "error": "not found" }))
    }
}
