use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    pub username: Option<String>,
    pub password: String,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
}
