use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UserResponse {
    data: User,
}

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    id: u32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

// using json web tokens from this
// https://github.com/Keats/jsonwebtoken/blob/master/examples/validation.rs

pub async fn create_user(Json(request_user): Json<RequestUser>) -> Json<UserResponse> {
    let user_response = UserResponse {
        data: User {
            username: request_user.username,
            ..Default::default()
        },
    };

    Json(user_response)
}
