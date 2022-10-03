use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_authenticated: bool,
}
