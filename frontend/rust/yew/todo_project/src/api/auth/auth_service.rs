use reqwasm::{http::{Method, Headers}, Error};
use serde_json::json;

use crate::api::{api_client::ApiClient, api_error::ApiError};

use super::{auth_response::AuthResponse, auth::Auth};

pub struct AuthService;

const LOGIN_URI: &str = "/users/login";
const USERS_URI: &str = "/users";

impl AuthService {
    pub async fn login(username: String, password: String) -> Result<Auth, String> {
        let response: Result<Result<AuthResponse, ApiError>, Error> = ApiClient::send_json(
            LOGIN_URI,
            Method::POST,
        Some(AuthService::get_auth_body(username, password)),
        Some(AuthService::get_headers())).await;

        return match response {
            Ok(ok) => match ok {
                Ok(ok) => Ok(ok.data),
                Err(err) => Err(err.error),
            },
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn register(username: String, password: String) -> Result<Auth, String> {
        let response: Result<Result<AuthResponse, ApiError>, Error> = ApiClient::send_json(
            USERS_URI,
            Method::POST,
            Some(AuthService::get_auth_body(username, password)),
            Some(AuthService::get_headers()),
        )
        .await;

        return match response {
            Ok(ok) => match ok {
                Ok(ok) => Ok(ok.data),
                Err(err) => Err(err.error),
            },
            Err(err) => Err(err.to_string()),
        }
    }

    fn get_auth_body(username: String, password: String) -> String {
        json! {
            {
                "username": username,
                "password": password
            }
        }
        .to_string()
    }

    fn get_headers() -> Headers {
        let headers = Headers::default();
        headers.append("content-type", "application/json");
        headers
}

}
