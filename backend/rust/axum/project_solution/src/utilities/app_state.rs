use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
}
