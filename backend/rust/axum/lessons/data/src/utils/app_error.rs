use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}
