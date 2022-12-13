pub mod create_task;
mod create_task_extractor;
pub mod get_all_tasks;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestTask {
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}
