use serde::{Serialize, Deserialize};

use super::task::Task;

#[derive(Serialize, Deserialize)]
pub struct TasksResponse {
    pub data: Vec<Task>
}

#[derive(Serialize, Deserialize)]
pub struct TaskResponse {
    pub data: Task
}
