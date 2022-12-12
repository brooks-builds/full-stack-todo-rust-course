use axum::{extract::State, http::StatusCode, response::Response, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

use crate::{
    database::{tasks, users::Model as UserModel},
    utilities::app_error::AppError,
};

use super::{
    create_task_extractor::ValidateCreateTask, RequestTask, ResponseDataTask, ResponseTask,
};

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    // let new_task = tasks::ActiveModel {
    //     priority: Set(request_user.priority),
    //     title: Set(request_user.title),
    //     description: Set(request_user.description),
    //     user_id: Set(Some(user.id)),
    //     ..Default::default()
    // };

    // let task = new_task
    //     .save(&db)
    //     .await
    //     .map_err(|error| {
    //         eprintln!("Error creating new task: {:?}", error);
    //         AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
    //     })?
    //     .try_into_model()
    //     .map_err(|error| {
    //         eprintln!("Error converting task after creating it: {:?}", error);
    //         AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
    //     })?;

    // let response = ResponseTask {
    //     id: task.id,
    //     title: task.title,
    //     description: task.description,
    //     priority: task.priority,
    //     completed_at: task.completed_at.map(|time| time.to_string()),
    // };

    // Ok((
    //     StatusCode::CREATED,
    //     Json(ResponseDataTask { data: response }),
    // ))
    todo!()
}
