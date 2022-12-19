use crate::{database::users::Model, queries::task_queries, utilities::app_error::AppError};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use chrono::Utc;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use super::RequestTask;

pub async fn mark_completed(
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut task = task_queries::find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    let now = Utc::now();
    task.completed_at = Set(Some(now.into()));

    task_queries::save_active_task(&db, task).await?;

    Ok(())
}

pub async fn mark_uncompleted(
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut task = task_queries::find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    task.completed_at = Set(None);

    task_queries::save_active_task(&db, task).await?;

    Ok(())
}

pub async fn update_task(
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), AppError> {
    let mut task = task_queries::find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    }

    if let Some(title) = request_task.title {
        task.title = Set(title);
    }

    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    if let Some(description) = request_task.description {
        task.description = Set(description);
    }

    task_queries::save_active_task(&db, task).await?;

    Ok(())
}
