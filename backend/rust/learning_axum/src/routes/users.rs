use crate::db::tasks::{self, Entity as Tasks};
use crate::utilities::errors::AppError;
use crate::utilities::hash_password::hash_password;
use crate::{config::Config, db::users, utilities::jwt::create_token};
use axum::http::StatusCode;
use axum::{Extension, Json};
use eyre::bail;
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
) -> Result<Json<UserResponse>, AppError> {
    let token = create_token(&config.jwt_secret, &request_user.username).unwrap();
    let hash = match hash_password(&request_user.password) {
        Ok(hash) => hash,
        Err(error) => return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error)),
    };
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(hash),
        token: Set(Some(token)),
        ..Default::default()
    };
    let user = match new_user.insert(&db).await {
        Ok(user) => user,
        Err(error) => {
            let code = match error.clone() {
                sea_orm::DbErr::Query(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(AppError::new(code, eyre::eyre!(error)));
        }
    };
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

    let result = Json(user_response);
    Ok(result)
}
