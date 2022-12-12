use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::{database::users, utilities::app_error::AppError};

pub async fn logout(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    user.save(&db).await.map_err(|error| {
        eprintln!("Error removing token from user: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was a problem logging out, please try again",
        )
    })?;

    Ok(StatusCode::OK)
}
