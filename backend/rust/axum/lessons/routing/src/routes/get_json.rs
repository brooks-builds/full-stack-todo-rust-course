use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "I'm in data!".to_owned(),
        count: 4857,
        username: "brookzerker".to_owned(),
    };

    Json(data)
}
