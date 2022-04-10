use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    Request::post("http://localhost:3000/api/v1/users")
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
