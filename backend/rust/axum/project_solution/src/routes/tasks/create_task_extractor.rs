use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::FromRequest,
    http::{Request, StatusCode},
    response::IntoResponse,
    BoxError, Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

use crate::utilities::app_error::AppError;

#[derive(Debug, Validate, Deserialize)]
pub struct ValidateCreateTask {
    #[validate(length(min = 1, max = 1))]
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ValidateCreateTask
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(task) = req
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|error| {
                eprintln!("Error extracting new task: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again",
                )
            })?;

        if let Err(errors) = task.validate() {
            dbg!(errors);
        }

        // Ok(Self(body))
        todo!()
    }
}
