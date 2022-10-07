use crate::db::tasks::{self, Entity as Tasks};
use crate::db::users::{self, Entity as Users};
use crate::utilities::errors::AppError;
use crate::utilities::hash_password::{hash_password, verify};
use crate::{config::Config, utilities::jwt::create_token};
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
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
            return Err(translate_error(error));
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

pub async fn sign_in(
    Json(request_user): Json<RequestUser>,
    Extension(db): Extension<DatabaseConnection>,
    Extension(config): Extension<Arc<Config>>,
) -> Result<Json<UserResponse>, AppError> {
    let db_user = match Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                eyre::eyre!("incorrect username and/or password"),
            ))
        }
        _ => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                eyre::eyre!("Internal Server error please try again later"),
            ))
        }
    };

    let is_password_correct = match verify(&request_user.password, &db_user.password) {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    if is_password_correct {
        let new_token = match create_token(&config.jwt_secret, &db_user.username) {
            Ok(token) => token,
            Err(error) => return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error)),
        };
        update_user_token(&new_token, db_user.clone().into(), &db).await?;
        Ok(Json(UserResponse {
            data: User {
                id: db_user.id,
                username: db_user.username,
                token: new_token,
            },
        }))
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            eyre::eyre!("incorrect username and/or password"),
        ))
    }
}

pub async fn logout(
    headers: HeaderMap,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<(), AppError> {
    let key = headers.get("x-auth-token").ok_or_else(|| {
        AppError::new(StatusCode::UNAUTHORIZED, eyre::eyre!("not authenticated!"))
    })?;
    let user = match Users::find()
        .filter(users::Column::Token.eq(Some(key.to_str().unwrap())))
        .one(&db)
        .await
    {
        Ok(user) => user,
        Err(error) => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                eyre::eyre!(error),
            ))
        }
    };

    if let Some(user) = user {
        let mut user: users::ActiveModel = user.into();
        user.token = Set(None);
        if let Err(error) = user.save(&db).await {
            Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                eyre::eyre!(error),
            ))
        } else {
            Ok(())
        }
    } else {
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            eyre::eyre!("not authenticated!"),
        ))
    }
}

fn translate_error(error: DbErr) -> AppError {
    if let DbErr::Query(query_error) = error {
        if query_error.as_str() == "error returned from database: duplicate key value violates unique constraint \"users_username_key\"" {
            AppError::new(StatusCode::BAD_REQUEST, eyre::eyre!("Username already taken, try again with a different user name"))
        } else {
            dbg!("is a query error, but not exactly what we thought", &query_error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(query_error))
        }
    } else {
        dbg!("not a query error");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, eyre::eyre!(error))
    }
}

async fn update_user_token(
    token: &str,
    mut db_user: users::ActiveModel,
    db: &DatabaseConnection,
) -> Result<(), AppError> {
    db_user.token = Set(Some(token.to_owned()));
    if let Err(error) = db_user.update(db).await {
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            eyre::eyre!(error),
        ))
    } else {
        Ok(())
    }
}
