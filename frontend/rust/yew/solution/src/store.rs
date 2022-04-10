use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

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
