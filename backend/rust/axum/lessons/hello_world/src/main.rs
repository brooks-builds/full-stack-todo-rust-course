use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    let listener = TcpListener::bind(&"0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}

async fn hello_world() -> String {
    "Hello world!".to_owned()
}
