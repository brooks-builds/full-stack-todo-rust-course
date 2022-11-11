mod create_task;
mod custom_json_extractor;
mod hello_world;
mod validate_with_serde;

use axum::{
    body::Body,
    routing::{get, post},
    Extension, Router,
};
use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use sea_orm::DatabaseConnection;
use validate_with_serde::validate_with_serde;

pub async fn create_routes(database: DatabaseConnection) -> Router<Body> {
    Router::new()
        .route("/hello_world", get(hello_world::hello_world))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .layer(Extension(database))
}
