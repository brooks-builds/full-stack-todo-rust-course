use axum::{
    routing::{get, post},
    Router,
};

pub mod hello_world;
mod users;

use users::create_user;

pub fn create_router() -> Router {
    Router::new()
        .route("/hello_world", get(hello_world::hello_world))
        .route("/api/v1/users", post(create_user))
}
