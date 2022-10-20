pub mod config;
mod db;
mod routes;
mod utilities;

use config::Config;
use eyre::Result;
use sea_orm::{Database, DatabaseConnection};
use std::{net::SocketAddr, sync::Arc};

pub async fn run(config: Arc<Config>) -> Result<()> {
    let db: DatabaseConnection = Database::connect(&config.database_uri).await?;
    let app = routes::create_router(Arc::clone(&config), db);
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
