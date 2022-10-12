use serde::{Deserialize, Serialize, de::DeserializeOwned};
use reqwasm::{http::*, Error};
use lazy_static::lazy_static;
use wasm_bindgen::JsValue;

const API_CONFIG_RAW_JSON: &str = include_str!("api_settings.json");

lazy_static! {
    static ref API_CONFIG: ApiConfig = serde_json::from_str(API_CONFIG_RAW_JSON).unwrap();
}

#[derive(Serialize, Deserialize)]
struct ApiConfig {
    api_uri: String
}

pub struct ApiClient;

impl ApiClient {
    pub async fn send_text(uri: &str, method: Method, body: Option<impl Into<JsValue>>, headers: Option<impl Into<Headers>>) -> Result<String, Error> {
        let mut request = Request::new(&format!("{}{}", API_CONFIG.api_uri, uri)).method(method);
            
        if let Some(body) = body {
            request = request.body(body);
        }

        if let Some(headers) = headers {
            request = request.headers(headers.into());
        }

        return match request.send().await {
            Ok(response) => Ok(response.text().await.unwrap()),
            Err(error) => Err(error),
        };
    }


    pub async fn send_json<T, E>(uri: &str, method: Method, body: Option<impl Into<JsValue>>, headers: Option<impl Into<Headers>>) -> Result<Result<T, E>, Error>
    where 
        T: DeserializeOwned,
        E: DeserializeOwned {
        let mut request = Request::new(&format!("{}{}", API_CONFIG.api_uri, uri)).method(method);
            
        if let Some(body) = body {
            request = request.body(body);
        }

        if let Some(headers) = headers {
            request = request.headers(headers.into());
        }

        return match request.send().await {
            Ok(response) => ApiClient::parse_response(response).await,
            Err(err) => Err(err)
        }
    }

    async fn parse_response<T, E>(response: Response) -> Result<Result<T, E>, Error>
    where 
        T: DeserializeOwned,
        E: DeserializeOwned {
            let response_clone = Response::from_raw(response.as_raw().clone().unwrap());
            return match response_clone.json::<T>().await {
                Ok(ok) => Ok(Ok(ok)),
                Err(_) => {
                    let response_clone = Response::from_raw(response.as_raw().clone().unwrap());
                    match response_clone.json::<E>().await {
                        Ok(ok) => Ok(Err(ok)),
                        Err(err) => Err(err),
                }}
            };
    }
}