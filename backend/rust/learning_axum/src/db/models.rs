use diesel::prelude::*;

#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub deleted_at: Option<String>,
    pub token: Option<String>,
}

#[derive(Queryable)]
pub struct Tasks {
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<String>,
    pub description: Option<String>,
    pub deleted_at: Option<String>,
    pub user_id: i32,
    pub is_default: Option<bool>,
}
