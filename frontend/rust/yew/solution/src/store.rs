use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::AuthResponse;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Store {
    pub username: String,
    pub token: String,
}

impl Persistent for Store {
    fn key() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn area() -> Area {
        Area::Local
    }
}

pub fn login_reducer(auth_response: AuthResponse, dispatch: Dispatch<PersistentStore<Store>>) {
    dispatch.reduce(move |store| {
        store.username = auth_response.data.username;
        store.token = auth_response.data.token;
    });
}
