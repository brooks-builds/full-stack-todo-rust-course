use super::{RequestCreateUser, ResponseDataUser, ResponseUser};
use crate::{
    database::users,
    utilities::{app_error::AppError, app_state::AppState},
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username);
    new_user.password = Set(request_user.password);
    let user = new_user
        .save(&db)
        .await
        .map_err(|error| {
            eprintln!("Error creating user: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            )
        })?
        .try_into_model()
        .map_err(|error| {
            eprintln!("Error converting user back into model: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating user")
        })?;
    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
        },
    }))
}
