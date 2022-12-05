use super::{RequestCreateUser, ResponseDataUser, ResponseUser};
use crate::database::users;
use axum::Json;
use sea_orm::{Set, ActiveModelTrait};

pub async fn create_user(Json(request_user): Json<RequestCreateUser>) -> Json<ResponseUser> {
    let mut new_user = users::ActiveModel::default();
    new_user.username = Set(request_user.username);
    new_user.password = Set(request_user.password);
    new_user.save(db)
    todo!()
}
