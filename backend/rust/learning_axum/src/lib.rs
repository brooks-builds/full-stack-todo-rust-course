pub mod config;
mod routes;
mod utilities;

use config::Config;
use eyre::Result;
use std::{net::SocketAddr, sync::Arc};

pub async fn run(config: Arc<Config>) -> Result<()> {
    let app = routes::create_router(Arc::clone(&config));
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
