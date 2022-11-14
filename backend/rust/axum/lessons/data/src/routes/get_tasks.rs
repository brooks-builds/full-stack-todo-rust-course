use crate::database::tasks::{self, Entity as Tasks};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id).one(&database).await.unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct GetAllQueryParams {
    priority: Option<String>,
}

pub async fn get_all_tasks(
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<GetAllQueryParams>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut filter_by_priority = Condition::all();
    if let Some(priority) = query_params.priority {
        filter_by_priority = filter_by_priority.add(tasks::Column::Priority.eq(Some(priority)));
    }

    let tasks = Tasks::find()
        .filter(filter_by_priority)
        // .filter(tasks::Column::Priority.eq(Some("A")))
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect();

    Ok(Json(tasks))
}
