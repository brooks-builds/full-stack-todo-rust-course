use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
