use crate::database::tasks::{self, Entity as Tasks};
use crate::{database::users::Model as UserModel, utilities::app_error::AppError};
use axum::http::StatusCode;
use axum::{extract::State, Extension, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::{ResponseDataTasks, ResponseTask};

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = Tasks::find()
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting all tasks: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
        })?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            description: db_task.description,
            priority: db_task.priority,
            completed_at: db_task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(ResponseDataTasks { data: tasks }))
}
