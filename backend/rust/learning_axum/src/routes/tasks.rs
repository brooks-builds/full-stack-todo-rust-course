use crate::db;
use crate::db::users::{self, Entity as Users, Model};
use crate::{
    db::tasks::{self, Entity as Tasks},
    utilities::errors::AppError,
};
use axum::body::HttpBody;
use axum::extract::FromRequest;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::BoxError;
use axum::{
    http::{HeaderMap, StatusCode},
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
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
    request_task: RequestTask,
    Extension(db): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<Json<ResponseOneTask>, AppError> {
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
