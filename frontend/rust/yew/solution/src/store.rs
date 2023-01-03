use crate::{
    api::{patch_task::PatchTask, AuthResponse, TaskResponse},
    components::atoms::bb_select::SelectOption,
};
use gloo::console;
use js_sys::Date;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Clone, Serialize, Deserialize, Store, PartialEq, Debug)]
#[store(storage = "local")]
pub struct Store {
    pub username: String,
    pub token: String,
    pub tasks: Vec<Task>,
    pub filter_options: Vec<SelectOption>,
    pub sort_options: Vec<SelectOption>,
    pub error_message: String,
}

impl Store {
    pub fn get_task_by_id(&self, id: u32) -> Option<Task> {
        self.tasks.iter().find(|task| task.id == id).cloned()
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
            error_message: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Task {
    pub completed_at: Option<String>,
    pub description: Option<String>,
    pub id: u32,
    pub priority: Option<String>,
    pub title: String,
}

pub fn login_reducer(auth_response: AuthResponse, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.username = auth_response.data.username;
        store.token = auth_response.data.token;
    });
}

pub fn set_tasks(tasks: TaskResponse, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.tasks = tasks.data;
    })
}

pub fn logout(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(|store| {
        store.username = String::new();
        store.token = String::new();
        store.tasks = vec![];
    });
}

pub fn update_task_by_id(dispatch: Dispatch<Store>, task_id: u32, patch_task: PatchTask) {
    dispatch.reduce_mut(move |store| {
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

pub fn remove_task_by_id(dispatch: Dispatch<Store>, task_id: u32) {
    dispatch.reduce_mut(move |store| {
        let store_tasks = store.tasks.clone();
        let tasks: Vec<Task> = store_tasks
            .into_iter()
            .filter(|task| task.id != task_id)
            .collect();
        store.tasks = tasks;
    })
}

pub fn add_task(dispatch: Dispatch<Store>, task: Task) {
    dispatch.reduce_mut(move |store| {
        store.tasks.push(task);
    });
}

pub fn mark_task_completed(dispatch: Dispatch<Store>, task_id: u32) {
    dispatch.reduce_mut(move |store| {
        let task = store.tasks.iter_mut().find(|task| task.id == task_id);
        if task.is_none() {
            gloo::console::error!("Error finding task to complete it");
            panic!();
        }
        let now = Date::new_0();
        task.unwrap().completed_at = now.to_utc_string().to_string().as_string();
    })
}

pub fn mark_task_uncompleted(dispatch: Dispatch<Store>, task_id: u32) {
    dispatch.reduce_mut(move |store| {
        let task = store.tasks.iter_mut().find(|task| task.id == task_id);
        if task.is_none() {
            gloo::console::error!("Error finding task to complete it");
            panic!();
        }
        task.unwrap().completed_at = None;
    })
}

pub fn select_filter(dispatch: Dispatch<Store>, filter_value: String) {
    dispatch.reduce_mut(move |store| {
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

pub fn select_sort(dispatch: Dispatch<Store>, sort_value: String) {
    dispatch.reduce_mut(move |store| {
        store.sort_options.iter_mut().for_each(move |sort_option| {
            if sort_option.value == sort_value {
                sort_option.is_selected = true;
            } else {
                sort_option.is_selected = false;
            }
        });
    })
}

pub fn reset_error_message(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(|store| {
        store.error_message = String::new();
    });
}

pub fn set_error_message(dispatch: Dispatch<Store>, error_message: &str) {
    let error_message = error_message.to_owned();
    dispatch.reduce_mut(move |store| store.error_message = error_message);
}
