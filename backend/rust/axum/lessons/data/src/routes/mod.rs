mod hello_world;

use axum::{body::Body, routing::get, Router};

pub async fn create_routes() -> Router<Body> {
    Router::new().route("/hello_world", get(hello_world::hello_world))
}
