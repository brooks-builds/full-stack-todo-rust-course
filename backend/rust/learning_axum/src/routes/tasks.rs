use std::str::FromStr;

use crate::db::users::Model;
use crate::{
    db::tasks::{self, Entity as Tasks},
    utilities::errors::AppError,
};
use axum::body::HttpBody;
use axum::extract::{FromRequest, Path};
use axum::BoxError;
use axum::{http::StatusCode, Extension, Json};
use chrono::{DateTime, FixedOffset, ParseError, Utc};
use sea_orm::prelude::{
    ChronoDateTime, ChronoDateTimeUtc, ChronoDateTimeWithTimeZone, DateTimeUtc,
    DateTimeWithTimeZone, TimeDate,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

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
    completed_at: Option<String>,
}

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
) -> Result<(), AppError> {
    let mut task = Tasks::find_by_id(task_id)
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
) -> Result<(), AppError> {
    let mut task = Tasks::find_by_id(task_id)
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
) -> Result<(), AppError> {
    let mut task = Tasks::find_by_id(task_id)
        .one(&db)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error)))?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, eyre::eyre!("not found!")))?
        .into_active_model();

    if let Some(completed_at) = request_task.completed_at {
        let date = DateTimeWithTimeZone::from_str(&completed_at).map_err(|error: ParseError| {
            AppError::new(StatusCode::UNPROCESSABLE_ENTITY, eyre::eyre!(error))
        })?;
        task.completed_at = Set(Some(date));
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
