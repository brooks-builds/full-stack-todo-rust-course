use std::net::SocketAddr;

use router::create_router;
use sea_orm::{Database, DatabaseConnection};
use utilities::app_state::{self, AppState};

mod database;
mod router;
mod routes;
mod utilities;

pub async fn run(database_url: String) {
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let app_state = AppState { db };
    let app = create_router(app_state);
    let address = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
