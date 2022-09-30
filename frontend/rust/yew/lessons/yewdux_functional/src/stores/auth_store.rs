use yewdux::prelude::*;

#[derive(Store, PartialEq, Clone, Default)]
pub struct AuthStore {
    pub username: String,
    pub password: String,
    pub token: Option<String>,
}
