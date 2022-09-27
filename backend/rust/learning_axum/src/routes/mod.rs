use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};

pub mod hello_world;
mod tasks;
mod users;

use sea_orm::DatabaseConnection;
use users::create_user;

use crate::config::Config;

use self::users::{logout, sign_in};

pub fn create_router(config: Arc<Config>, db: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello_world", get(hello_world::hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(sign_in))
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/tasks", get(tasks::get_all_tasks))
        .layer(Extension(config))
        .layer(Extension(db))
}
