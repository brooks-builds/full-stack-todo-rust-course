use axum::extract::State;

pub async fn middleware_message(State(message): State<String>) -> String {
    message
}
