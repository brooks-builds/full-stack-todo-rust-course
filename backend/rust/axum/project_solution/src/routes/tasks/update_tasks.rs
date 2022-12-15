use crate::{
    database::{
        tasks::{self, Entity as Tasks},
        users::Model,
    },
    utilities::app_error::AppError,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use super::RequestTask;

pub async fn mark_completed(
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting task to update it: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happened")
        })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    let now = Utc::now();
    task.completed_at = Set(Some(now.into()));
    task.save(&db).await.map_err(|error| {
        eprintln!("Error marking task as completed: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating completed at",
        )
    })?;

    Ok(())
}

pub async fn mark_uncompleted(
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting task to update it: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happened")
        })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    task.completed_at = Set(None);
    task.save(&db).await.map_err(|error| {
        eprintln!("Error marking task as completed: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating completed at",
        )
    })?;

    Ok(())
}

pub async fn update_task(
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting task to update it: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happened")
        })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

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

    task.save(&db).await.map_err(|error| {
        eprintln!("Error marking task as completed: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating completed at",
        )
    })?;

    Ok(())
}
