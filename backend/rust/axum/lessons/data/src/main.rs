use data::run;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    run(database_uri).await;
}
