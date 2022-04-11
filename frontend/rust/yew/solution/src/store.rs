use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::{AuthResponse, TaskResponse};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Store {
    pub username: String,
    pub token: String,
    pub tasks: Vec<Task>,
}

impl Persistent for Store {
    fn key() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn area() -> Area {
        Area::Local
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Task {
    pub completed_at: Option<String>,
    pub description: Option<String>,
    pub id: u32,
    pub priority: Option<String>,
    pub title: String,
}

pub fn login_reducer(auth_response: AuthResponse, dispatch: Dispatch<PersistentStore<Store>>) {
    dispatch.reduce(move |store| {
        store.username = auth_response.data.username;
        store.token = auth_response.data.token;
    });
}

pub fn set_tasks(dispatch: Dispatch<PersistentStore<Store>>) {
    // dispatch.reduce(move |store| {
    //     store.tasks = task_response.data;
    // })
}
