use crate::api::{patch_task::PatchTask, AuthResponse, TaskResponse};
use gloo::console;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

pub type StoreDispatch = Dispatch<StoreType>;
pub type StoreType = PersistentStore<Store>;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Store {
    pub username: String,
    pub token: String,
    pub tasks: Vec<Task>,
}

impl Store {
    pub fn get_task_by_id(&self, id: u32) -> Option<Task> {
        match self.tasks.iter().find(|task| task.id == id) {
            Some(task) => Some(task.clone()),
            None => None,
        }
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
    pub priority: Option<String>,
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

pub fn logout(dispatch: StoreDispatch) {
    dispatch.reduce(|store| {
        store.username = String::new();
        store.token = String::new();
    });
}

pub fn update_task_by_id(dispatch: StoreDispatch, task_id: u32, patch_task: PatchTask) {
    dispatch.reduce(move |store| {
        let task = store.tasks.iter_mut().find(|task| task.id == task_id);
        let task = if let Some(task) = task {
            task
        } else {
            console::error!("Could not find task in Yewdux store");
            panic!();
        };
        if let Some(title) = patch_task.title {
            task.title = title;
        }
        if let Some(completed_at) = patch_task.completed_at {
            task.completed_at = completed_at;
        }
        if patch_task.priority.is_some() {
            task.priority = patch_task.priority;
        }
        if patch_task.description.is_some() {
            task.description = patch_task.description;
        }
    })
}

pub fn remove_task_by_id(dispatch: StoreDispatch, task_id: u32) {
    dispatch.reduce(move |store| {
        let store_tasks = store.tasks.clone();
        let tasks: Vec<Task> = store_tasks
            .into_iter()
            .filter(|task| task.id != task_id)
            .collect();
        store.tasks = tasks;
    })
}

pub fn add_task(dispatch: StoreDispatch, task: Task) {
    dispatch.reduce(move |store| {
        store.tasks.push(task);
    });
}
