use dotenvy::dotenv;
use project_solution::{app_state::AppState, run, utilities::token_wrapper::TokenWrapper};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("Missing environment variable DATABASE_URL")
        .to_owned();
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("Missing environment variable JWT_SECRET")
        .to_owned();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let app_state = AppState {
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };

    run(app_state).await;
}
