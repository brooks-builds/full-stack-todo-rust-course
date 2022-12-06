use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
}

pub fn create_token() -> Result<String, AppError> {
    // add at least an hour for this timestamp
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {exp: now}
    todo!()
}
