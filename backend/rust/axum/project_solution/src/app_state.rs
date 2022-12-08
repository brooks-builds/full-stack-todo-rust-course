use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use crate::utilities::token_wrapper::{self, TokenWrapper};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_secret: TokenWrapper,
}
