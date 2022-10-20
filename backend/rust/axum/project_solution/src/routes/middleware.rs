use crate::db::users::{self, Entity as Users};
use crate::utilities::errors::AppError;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn auth_required<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let key = request.headers().get("x-auth-token").ok_or_else(|| {
        AppError::new(StatusCode::UNAUTHORIZED, eyre::eyre!("not authenticated!"))
    })?;
    let db = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                eyre::eyre!("Error getting database connection"),
            )
        })?;
    match Users::find()
        .filter(users::Column::Token.eq(Some(key.to_str().unwrap())))
        .one(db)
        .await
    {
        Ok(Some(user)) => {
            request.extensions_mut().insert(user);
            Ok(next.run(request).await)
        }
        Ok(None) => {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                eyre::eyre!("not authenticated!"),
            ));
        }
        Err(error) => {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                eyre::eyre!(error),
            ));
        }
    }
}
