use std::sync::Arc;

use crate::{config::Config, utilities::jwt::create_token};
use axum::{Extension, Json};
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

pub async fn create_user(
    Json(request_user): Json<RequestUser>,
    Extension(config): Extension<Arc<Config>>,
) -> Json<UserResponse> {
    let token = create_token(&config.jwt_secret, &request_user.username).unwrap();
    let user_response = UserResponse {
        data: User {
            username: request_user.username,
            token,
            ..Default::default()
        },
    };

    Json(user_response)
}
