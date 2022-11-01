use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await;
}
