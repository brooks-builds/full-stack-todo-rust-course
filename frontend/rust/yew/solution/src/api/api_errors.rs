use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Expired or missing auth token")]
    NotAuthenticated,
    #[error("Unknown Network error")]
    Unknown,
}
