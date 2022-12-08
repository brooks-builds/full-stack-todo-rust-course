use super::app_error::AppError;
use axum::http::StatusCode;
use bcrypt::hash;

const COST: u32 = 12;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, COST).map_err(|error| {
        eprintln!("Error hashing password: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error securing password")
    })
}
