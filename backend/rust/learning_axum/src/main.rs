#[tokio::main]
async fn main() {
    match learning_axum::run(3000).await {
        Ok(_) => println!("app running on port 3000"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}
