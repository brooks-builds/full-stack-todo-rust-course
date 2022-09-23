use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub struct AppError {
    status_code: StatusCode,
    error: eyre::Error,
}

impl AppError {
    pub fn new(status_code: StatusCode, error: eyre::Error) -> Self {
        Self { status_code, error }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            Json(ErrorResponse {
                error: format!("error: {:?}", self.error),
            }),
        )
            .into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}
