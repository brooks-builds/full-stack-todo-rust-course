use axum::{body::HttpBody, extract::FromRequest, http::StatusCode, BoxError, Json};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

use crate::utilities::errors::AppError;

pub mod routes;

#[derive(Serialize, Deserialize)]
pub struct JsonRequestTask {
    pub priority: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub struct RequestTask {
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

impl<T> FromRequest<T> for RequestTask
where
    T: HttpBody + Send,
    T::Data: Send,
    T::Error: Into<BoxError>,
{
    type Rejection = AppError;

    fn from_request<'life0, 'async_trait>(
        req: &'life0 mut axum::extract::RequestParts<T>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let Json(body) = req
                .extract::<Json<JsonRequestTask>>()
                .await
                .map_err(|_error| AppError::new(StatusCode::BAD_REQUEST, eyre::eyre!("error")))?;

            let title = body.title.ok_or_else(|| {
                AppError::new(StatusCode::BAD_REQUEST, eyre::eyre!("missing task title"))
            })?;

            Ok(Self {
                priority: body.priority,
                title,
                description: body.description,
            })
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct TaskResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdateTask {
    pub priority: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
}
