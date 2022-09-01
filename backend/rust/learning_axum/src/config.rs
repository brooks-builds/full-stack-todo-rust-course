use std::sync::Arc;

pub struct Config {
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn new(jwt_secret: String, port: u16) -> Arc<Self> {
        Arc::new(Self { jwt_secret, port })
    }
}
