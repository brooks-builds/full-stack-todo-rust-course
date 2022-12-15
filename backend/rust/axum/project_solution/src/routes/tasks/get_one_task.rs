use crate::database::tasks::{self, Entity as Tasks};
use crate::{database::users::Model, utilities::app_error::AppError};
use axum::Json;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::{ResponseDataTask, ResponseTask};

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error when getting one task: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "We got an error when attempting to load your task",
            )
        })?;

    let response_task = if let Some(task) = task {
        ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            completed_at: task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
        }
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    Ok(Json(ResponseDataTask {
        data: response_task,
    }))
}
