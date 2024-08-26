use app_state::AppState;
use router::create_router;
use tokio::net::TcpListener;

pub mod app_state;
mod database;
mod middleware;
mod queries;
mod router;
mod routes;
pub mod utilities;

pub async fn run(app_state: AppState) {
    let app = create_router(app_state);
    let address = TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(address, app.into_make_service()).await.unwrap();
}
