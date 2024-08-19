mod database;
mod routes;
mod utils;

use sea_orm::Database;
use tokio::net::TcpListener;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    let app = routes::create_routes(database).await;
    let listener = TcpListener::bind(&"0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
