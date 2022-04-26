use crate::{
    api::{patch_task::PatchTask, AuthResponse, TaskResponse},
    components::atoms::bb_select::SelectOption,
};
use gloo::console;
use js_sys::Date;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

pub type StoreDispatch = Dispatch<StoreType>;
pub type StoreType = PersistentStore<Store>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Store {
    pub username: String,
    pub token: String,
    pub tasks: Vec<Task>,
    pub filter_options: Vec<SelectOption>,
    pub sort_options: Vec<SelectOption>,
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

impl Default for Store {
    fn default() -> Self {
        Self {
            username: Default::default(),
            token: Default::default(),
            tasks: Default::default(),
            filter_options: vec![
                SelectOption::new("none", "None", true),
                SelectOption::new("completed", "Completed", false),
                SelectOption::new("uncompleted", "Uncompleted", false),
                SelectOption::new("priority_a", "Priority A", false),
                SelectOption::new("priority_b", "Priority B", false),
                SelectOption::new("priority_c", "Priority C", false),
            ],
            sort_options: vec![
                SelectOption::new("created_order", "Created Order", true),
                SelectOption::new("priority", "Priority", false),
                SelectOption::new("name", "Name", false),
            ],
        }
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

pub fn mark_task_completed(dispatch: StoreDispatch, task_id: u32) {
    dispatch.reduce(move |store| {
        let task = store.tasks.iter_mut().find(|task| task.id == task_id);
        if task.is_none() {
            gloo::console::error!("Error finding task to complete it");
            panic!();
        }
        let now = Date::new_0();
        task.unwrap().completed_at = now.to_utc_string().to_string().as_string();
    })
}

pub fn mark_task_uncompleted(dispatch: StoreDispatch, task_id: u32) {
    dispatch.reduce(move |store| {
        let task = store.tasks.iter_mut().find(|task| task.id == task_id);
        if task.is_none() {
            gloo::console::error!("Error finding task to complete it");
            panic!();
        }
        task.unwrap().completed_at = None;
    })
}

pub fn select_filter(dispatch: StoreDispatch, filter_value: String) {
    dispatch.reduce(move |store| {
        store
            .filter_options
            .iter_mut()
            .for_each(move |filter_option| {
                if filter_option.value == filter_value {
                    filter_option.is_selected = true;
                } else {
                    filter_option.is_selected = false;
                }
            });
    })
}

pub fn select_sort(dispatch: StoreDispatch, sort_value: String) {
    dispatch.reduce(move |store| {
        store.sort_options.iter_mut().for_each(move |sort_option| {
            if sort_option.value == sort_value {
                sort_option.is_selected = true;
            } else {
                sort_option.is_selected = false;
            }
        });
    })
}
