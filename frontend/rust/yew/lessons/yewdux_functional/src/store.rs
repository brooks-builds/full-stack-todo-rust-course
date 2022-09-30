use yewdux::prelude::*;

#[derive(Clone, Default)]
pub struct YewduxStore {
    pub username: String,
    pub password: String,
    pub token: String,
}
