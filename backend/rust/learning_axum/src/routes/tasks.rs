use axum::{Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::db::tasks::Entity as Tasks;

pub async fn get_all_tasks(
    Extension(db): Extension<DatabaseConnection>,
) -> Json<Vec<crate::db::tasks::Model>> {
    let all_tasks = Tasks::find().all(&db).await.unwrap();
    Json(all_tasks)
}
