use super::{RequestTask, RequestUpdateTask, Task, TaskResponse};
use crate::db::users::Model;
use crate::{
    db::tasks::{self, Entity as Tasks},
    utilities::errors::AppError,
};
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
use chrono::FixedOffset;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

pub async fn get_all_tasks(
    Extension(db): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<Json<TaskResponse<Vec<Task>>>, AppError> {
    let all_tasks = Tasks::find()
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?
        .into_iter()
        .map(|db_task| Task {
            id: db_task.id,
            priority: db_task.priority,
            title: db_task.title,
            completed_at: db_task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
            description: db_task.description,
        })
        .collect::<Vec<Task>>();

    Ok(Json(TaskResponse { data: all_tasks }))
}

pub async fn create_task(
    request_task: RequestTask,
    Extension(db): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<Json<TaskResponse<Task>>, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        is_default: Set(None),
        ..Default::default()
    };
    let created_task = match new_task.save(&db).await {
        Ok(task) => task,
        Err(error) => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                eyre::eyre!(error),
            ));
        }
    };
    Ok(Json(TaskResponse {
        data: Task {
            id: created_task.id.unwrap(),
            priority: created_task.priority.unwrap(),
            title: created_task.title.unwrap(),
            completed_at: created_task
                .completed_at
                .unwrap()
                .map(|completed_at| completed_at.to_string()),
            description: created_task.description.unwrap(),
        },
    }))
}

pub async fn get_one_task(
    Extension(user): Extension<Model>,
    Extension(db): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<Json<TaskResponse<Task>>, AppError> {
    let db_task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, eyre::eyre!("not found")))?;
    Ok(Json(TaskResponse {
        data: Task {
            id: db_task.id,
            priority: db_task.priority,
            title: db_task.title,
            completed_at: db_task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
            description: db_task.description,
        },
    }))
}

pub async fn mark_completed(
    Extension(db): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
) -> Result<(), AppError> {
    let mut task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, eyre::eyre!("not found")))?
        .into_active_model();
    let now = chrono::Local::now();
    let now = chrono::DateTime::<FixedOffset>::from(now);
    task.completed_at = Set(Some(now));

    task.save(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?;

    Ok(())
}

pub async fn mark_uncompleted(
    Extension(db): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
) -> Result<(), AppError> {
    let mut task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, eyre::eyre!("not found")))?
        .into_active_model();
    task.completed_at = Set(None);

    task.save(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?;

    Ok(())
}

pub async fn update(
    Extension(db): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(request_task): Json<RequestUpdateTask>,
    Extension(user): Extension<Model>,
) -> Result<(), AppError> {
    let mut task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, eyre::eyre!("not found!")))?
        .into_active_model();

    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    if let Some(description) = request_task.description {
        task.description = Set(Some(description));
    }

    if let Some(priority) = request_task.priority {
        task.priority = Set(Some(priority));
    }

    if let Some(title) = request_task.title {
        task.title = Set(title);
    }

    task.save(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?;

    Ok(())
}

pub async fn delete_task(
    Extension(user): Extension<Model>,
    Extension(db): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<(), AppError> {
    let mut task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, eyre::eyre!("not found!")))?
        .into_active_model();

    let now = chrono::DateTime::<FixedOffset>::from(chrono::Local::now());
    task.deleted_at = Set(Some(now));

    task.save(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?;

    Ok(())
}
