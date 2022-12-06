use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use project_solution::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL").to_owned();
    run(database_url).await;
}
