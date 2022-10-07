use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub error: String
}