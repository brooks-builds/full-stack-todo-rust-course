use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Debug)]
pub struct AuthStore {
    pub username: String,
    pub password: String
}
