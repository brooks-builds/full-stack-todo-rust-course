use dotenvy_macro::dotenv;
use learning_axum::config::Config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let jwt_secret = dotenv!("JWT_SECRET").to_owned();
    let port = dotenv!("PORT")
        .parse()
        .expect("unable to parse port into a number");
    let config = Config::new(jwt_secret, port);

    match learning_axum::run(config).await {
        Ok(_) => println!("app running on port 3000"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}
