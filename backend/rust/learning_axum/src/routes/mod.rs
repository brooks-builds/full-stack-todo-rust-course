pub mod hello_world;
mod middleware;
mod tasks;
mod users;

use self::{
    tasks::create_task,
    users::{logout, sign_in},
};
use crate::config::Config;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use middleware::auth_required;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use users::create_user;

pub fn create_router(config: Arc<Config>, db: DatabaseConnection) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/tasks", get(tasks::get_all_tasks))
        .route("/api/v1/tasks", post(create_task))
        .layer(axum::middleware::from_fn(auth_required))
        .route("/hello_world", get(hello_world::hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(sign_in))
        .layer(Extension(config))
        .layer(Extension(db))
}
