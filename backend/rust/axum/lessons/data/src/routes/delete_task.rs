use crate::database::tasks::{self, Entity as Tasks};
use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn delete_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    // let task = if let Some(task) = Tasks::find_by_id(task_id)
    //     .one(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    // {
    //     task.into_active_model()
    // } else {
    //     return Err(StatusCode::NOT_FOUND);
    // };

    // Tasks::delete(task)
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Tasks::delete_by_id(task_id)
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Tasks::delete_many()
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
