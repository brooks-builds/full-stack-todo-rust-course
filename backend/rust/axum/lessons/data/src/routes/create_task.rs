use axum::Extension;
use sea_orm::DatabaseConnection;

pub async fn create_task(Extension(database): Extension<DatabaseConnection>) {}
