use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(title: String) -> Task {
        Task {
            id: Uuid::new_v4(),
            title,
            done: false,
        }
    }
}
