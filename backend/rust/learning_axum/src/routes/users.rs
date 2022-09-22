use crate::db::tasks::{self, Entity as Tasks};
use crate::{config::Config, db::users, utilities::jwt::create_token};
use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Default)]
pub struct UserResponse {
    data: User,
}

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    id: i32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

pub async fn create_user(
    Json(request_user): Json<RequestUser>,
    Extension(config): Extension<Arc<Config>>,
    Extension(db): Extension<DatabaseConnection>,
) -> Json<UserResponse> {
    let token = create_token(&config.jwt_secret, &request_user.username).unwrap();
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some(token)),
        ..Default::default()
    };
    let user = new_user.insert(&db).await.unwrap();
    let default_tasks = Tasks::find()
        .filter(tasks::Column::IsDefault.eq(true))
        .all(&db)
        .await
        .unwrap();
    let new_user_id = user.id;
    for default_task in default_tasks {
        let new_task = tasks::ActiveModel {
            priority: Set(default_task.priority),
            title: Set(default_task.title),
            completed_at: Set(default_task.completed_at),
            description: Set(default_task.description),
            deleted_at: Set(default_task.deleted_at),
            user_id: Set(Some(new_user_id)),
            ..Default::default()
        };
        new_task.insert(&db).await.unwrap();
    }

    let user_response = UserResponse {
        data: User {
            id: user.id,
            username: user.username.clone(),
            token: user.token.clone().unwrap_or_default(),
        },
    };

    Json(user_response)
}
