mod hello_world;

use axum::{body::Body, routing::patch, Router};
use hello_world::hello_world;

pub fn create_routes() -> Router<Body> {
    Router::new().route("/", patch(hello_world))
}
