use super::{convert_active_to_model, RequestCreateUser, ResponseDataUser, ResponseUser};
use crate::database::tasks::{self, Entity as Tasks};
use crate::database::users::Model;
use crate::{
    database::users,
    utilities::{
        app_error::AppError, hash::hash_password, jwt::create_token, token_wrapper::TokenWrapper,
    },
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username.clone());
    new_user.password = Set(hash_password(&request_user.password)?);
    new_user.token = Set(Some(create_token(&jwt_secret.0, request_user.username)?));
    let user = new_user.save(&db).await.map_err(|error| {
        let error_message = error.to_string();

        if error_message
            .contains("duplicate key value violates unique constraint \"users_username_key\"")
        {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different user name",
            )
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            )
        }
    })?;
    let user = convert_active_to_model(user)?;

    create_default_tasks_for_user(&db, &user).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap(),
        },
    }))
}

async fn create_default_tasks_for_user(
    db: &DatabaseConnection,
    user: &Model,
) -> Result<(), AppError> {
    let default_tasks = Tasks::find()
        .filter(tasks::Column::IsDefault.eq(Some(true)))
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting default tasks: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error applying default tasks to new account",
            )
        })?;

    for default_task in default_tasks {
        let task = tasks::ActiveModel {
            priority: Set(default_task.priority),
            title: Set(default_task.title),
            completed_at: Set(default_task.completed_at),
            description: Set(default_task.description),
            deleted_at: Set(default_task.deleted_at),
            user_id: Set(Some(user.id)),
            ..Default::default()
        };

        task.save(db).await.map_err(|error| {
            eprintln!("Error creating task from default: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error saving new default task for user",
            )
        })?;
    }

    Ok(())
}
