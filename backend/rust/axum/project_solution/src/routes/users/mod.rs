use serde::{Deserialize, Serialize};

pub mod create_user;

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    username: String,
    password: String,
}