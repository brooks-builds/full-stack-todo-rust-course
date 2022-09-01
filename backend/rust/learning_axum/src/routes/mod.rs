use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};

pub mod hello_world;
mod users;

use users::create_user;

use crate::config::Config;

pub fn create_router(config: Arc<Config>) -> Router {
    Router::new()
        .route("/hello_world", get(hello_world::hello_world))
        .route("/api/v1/users", post(create_user))
        .layer(Extension(config))
}
