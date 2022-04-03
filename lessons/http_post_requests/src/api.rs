use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    let response = Request::post("http://localhost:3000/api/v1/users/login")
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
