mod routes;

use eyre::Result;
use std::net::SocketAddr;

pub async fn run(port: u16) -> Result<()> {
    let app = routes::create_router();
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
