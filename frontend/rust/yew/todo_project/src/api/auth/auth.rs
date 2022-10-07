use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Auth {
    pub id: u32,
    pub username: String,
    pub token: String,
}
