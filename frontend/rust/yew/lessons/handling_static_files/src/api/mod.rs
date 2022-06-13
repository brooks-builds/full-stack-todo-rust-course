use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

const BASE_API_URI: &str = include_str!("base_api_uri.txt");

#[derive(Serialize, Deserialize)]
pub struct ApiLoginResponse {
    pub id: u32,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
struct ApiLoginResponseData {
    pub data: ApiLoginResponse,
}

pub async fn api_login(username: String, password: String) -> ApiLoginResponse {
    let body = json!({
      "username": username,
      "password": password
    });
    let response = Request::post(&format!("{}/users/login", BASE_API_URI))
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiLoginResponseData>()
        .await
        .unwrap();

    response.data
}
