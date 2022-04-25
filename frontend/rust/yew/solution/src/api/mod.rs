pub mod api_errors;
pub mod patch_task;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::store::Task;

use self::{api_errors::ApiError, patch_task::PatchTask};

// TODO refactor url to environment variable
const BASE_URL: &str = include_str!("api_base_uri.txt");

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub data: User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskResponse {
    pub data: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct SingleTaskResponse {
    pub data: Task,
}

pub async fn create_account(username: String, password: String) -> AuthResponse {
    Request::post(&format!("{}/users", BASE_URL))
        .header("Content-Type", "application/json")
        .body(
            json!({
              "username": username,
              "password": password
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap()
        .json::<AuthResponse>()
        .await
        .unwrap()
}

pub async fn login(username: String, password: String) -> AuthResponse {
    Request::post(&format!("{}/users/login", BASE_URL))
        .header("content-type", "application/json")
        .body(
            json! ({
                "username": username,
                "password": password
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap()
        .json::<AuthResponse>()
        .await
        .unwrap()
}

pub async fn get_tasks(token: &str) -> Result<TaskResponse, ApiError> {
    let request = Request::get(&format!("{}/tasks", BASE_URL))
        .header("x-auth-token", token)
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<TaskResponse>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn update_task(task_id: u32, token: &str, task: PatchTask) -> Result<(), ApiError> {
    let request = Request::patch(&format!("{}/tasks/{}", BASE_URL, task_id))
        .header("x-auth-token", token)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&task).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn delete_task(task_id: u32, token: &str) -> Result<(), ApiError> {
    let request = Request::delete(&format!("{}/tasks/{}", BASE_URL, task_id))
        .header("x-auth-token", token)
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn create_task(
    token: &str,
    title: String,
    description: Option<String>,
    priority: String,
) -> Result<SingleTaskResponse, ApiError> {
    let new_task = PatchTask::new(Some(title), Some(priority), description, None);
    let request = Request::post(&format!("{}/tasks", BASE_URL))
        .header("x-auth-token", token)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&new_task).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(request.json::<SingleTaskResponse>().await.unwrap())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn complete_task(task_id: u32, token: &str) -> Result<(), ApiError> {
    let request = Request::put(&format!("{}/tasks/{}/completed", BASE_URL, task_id))
        .header("x-auth-token", token)
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn uncomplete_task(task_id: u32, token: &str) -> Result<(), ApiError> {
    let request = Request::put(&format!("{}/tasks/{}/uncompleted", BASE_URL, task_id))
        .header("x-auth-token", token)
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

fn handle_errors(status: u16) -> ApiError {
    match status {
        401 => ApiError::NotAuthenticated,
        _ => ApiError::Unknown,
    }
}
