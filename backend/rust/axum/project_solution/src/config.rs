use std::sync::Arc;

pub struct Config {
    pub jwt_secret: String,
    pub port: u16,
    pub database_uri: String,
}

impl Config {
    pub fn new(jwt_secret: String, port: u16, database_uri: &str) -> Arc<Self> {
        Arc::new(Self {
            jwt_secret,
            port,
            database_uri: database_uri.to_owned(),
        })
    }
}
