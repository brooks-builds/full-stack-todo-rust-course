use axum::{routing::get, Router};

pub mod hello_world;

pub fn create_router() -> Router {
    Router::new().route("/hello_world", get(hello_world::hello_world))
}
