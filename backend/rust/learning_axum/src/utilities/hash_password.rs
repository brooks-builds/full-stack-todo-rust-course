use axum::http::StatusCode;
use dotenvy_macro::dotenv;

use super::errors::AppError;

pub fn hash_password(password: &str) -> eyre::Result<String> {
    let cost: &str = dotenv!("HASH_COST");
    let hash = bcrypt::hash(password, cost.parse()?)?;
    Ok(hash)
}

pub fn verify(password: &str, hash: &str) -> Result<bool, AppError> {
    match bcrypt::verify(password, hash) {
        Ok(is_verified) => Ok(is_verified),
        Err(error) => Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            eyre::eyre!(error),
        )),
    }
}
