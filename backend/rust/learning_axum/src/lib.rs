pub mod config;
mod db;
mod routes;
mod utilities;

use config::Config;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use eyre::Result;
use std::{net::SocketAddr, sync::Arc};

fn establish_connection(config: Arc<Config>) -> Result<PgConnection> {
    let connection = PgConnection::establish(&config.database_uri)?;
    Ok(connection)
}

pub async fn run(config: Arc<Config>) -> Result<()> {
    let database_connection = establish_connection(Arc::clone(&config))?;
    let app = routes::create_router(Arc::clone(&config));
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
