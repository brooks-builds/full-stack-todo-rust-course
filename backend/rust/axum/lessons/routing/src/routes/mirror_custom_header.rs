use axum::http::HeaderMap;

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let message_value = headers.get("User-Agent").unwrap();
    let message = message_value.to_str().unwrap().to_owned();
    message
}
