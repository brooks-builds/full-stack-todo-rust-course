use crate::db;
use crate::db::users::{self, Entity as Users};
use crate::{
    db::tasks::{self, Entity as Tasks},
    utilities::errors::AppError,
};
use axum::{
    http::{HeaderMap, StatusCode},
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestTask {
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseOneTask {
    pub data: Task,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<String>,
    pub description: Option<String>,
}

pub async fn get_all_tasks(
    Extension(db): Extension<DatabaseConnection>,
) -> Json<Vec<crate::db::tasks::Model>> {
    let all_tasks = Tasks::find().all(&db).await.unwrap();
    Json(all_tasks)
}

pub async fn create_task(
    Json(request_task): Json<RequestTask>,
    headers: HeaderMap,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseOneTask>, AppError> {
    let key = headers.get("x-auth-token").ok_or_else(|| {
        AppError::new(StatusCode::UNAUTHORIZED, eyre::eyre!("not authenticated!"))
    })?;
    let user = match Users::find()
        .filter(users::Column::Token.eq(Some(key.to_str().unwrap())))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                eyre::eyre!("Could not find user"),
            ));
        }
        Err(error) => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                eyre::eyre!(error),
            ));
        }
    };

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
    Ok(Json(ResponseOneTask {
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
