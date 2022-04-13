use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::{AuthResponse, TaskResponse};

pub type StoreDispatch = Dispatch<StoreType>;
pub type StoreType = PersistentStore<Store>;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Store {
    pub username: String,
    pub token: String,
    pub tasks: Vec<Task>,
}

impl Store {
    pub fn get_task_by_id(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }
}

impl Persistent for Store {
    fn key() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn area() -> Area {
        Area::Local
    }
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub completed_at: Option<String>,
    pub description: Option<String>,
    pub id: u32,
    pub priority: Option<char>,
    pub title: String,
}

pub fn login_reducer(auth_response: AuthResponse, dispatch: StoreDispatch) {
    dispatch.reduce(move |store| {
        store.username = auth_response.data.username;
        store.token = auth_response.data.token;
    });
}

pub fn set_tasks(tasks: TaskResponse, dispatch: StoreDispatch) {
    dispatch.reduce(move |store| {
        store.tasks = tasks.data;
    })
}
