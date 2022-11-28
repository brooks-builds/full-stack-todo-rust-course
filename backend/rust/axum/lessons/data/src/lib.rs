mod database;
mod routes;
mod utils;

use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    let app = routes::create_routes(database).await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
