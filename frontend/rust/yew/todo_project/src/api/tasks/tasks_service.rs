use reqwasm::{
    http::{Headers, Method},
    Error,
};

use crate::api::{api_client::ApiClient, api_error::ApiError};

use super::{task::Task, tasks_response::{TasksResponse, TaskResponse}};

pub struct TasksService;

const TASKS_URI: &str = "/tasks";

impl TasksService {
    pub async fn create_task(token: String, task: Task) -> Result<Task, String> {
        let response: Result<Result<TaskResponse, ApiError>, Error> = ApiClient::send_json(
            TASKS_URI,
            Method::POST,
            Some(serde_json::to_string(&task).unwrap()),
            Some(TasksService::get_headers(token)),
        )
        .await;

        return match response {
            Ok(ok) => match ok {
                Ok(ok) => Ok(ok.data),
                Err(err) => Err(err.error),
            },
            Err(err) => Err(err.to_string()),
        };
    }

    pub async fn update_task(token: String, task: Task) -> Result<(), String> {
        let response: Result<String, Error> = ApiClient::send_text(
            format!("{}/{}", TASKS_URI, &task.id).as_str(),
            Method::PATCH,
            Some(serde_json::to_string(&task).unwrap()),
            Some(TasksService::get_headers(token)),
        )
        .await;

        return match response {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        };
    }

    pub async fn delete_task(token: String, id: i32) -> Result<(), String> {
        let body: Option<&str> = None;
        let response: Result<String, Error> = ApiClient::send_text(
            format!("{}/{}", TASKS_URI, id).as_str(),
            Method::DELETE,
            body,
            Some(TasksService::get_headers(token)),
        )
        .await;

        return match response {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        };
    }

    pub async fn get_tasks(token: String) -> Result<Vec<Task>, String> {
        let body: Option<&str> = None;
        let response: Result<Result<TasksResponse, ApiError>, Error> = ApiClient::send_json(
            TASKS_URI,
            Method::GET,
            body,
            Some(TasksService::get_headers(token)),
        )
        .await;

        return match response {
            Ok(ok) => match ok {
                Ok(ok) => Ok(ok.data),
                Err(err) => Err(err.error),
            },
            Err(err) => Err(err.to_string()),
        };
    }

    fn get_headers(token: String) -> Headers {
        let headers = Headers::default();
        headers.append("content-type", "application/json");
        headers.append("x-auth-token", &token);
        headers
    }
}
