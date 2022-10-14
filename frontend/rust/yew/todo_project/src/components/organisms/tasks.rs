use chrono::Local;
use gloo::console::log;
use lazy_static::__Deref;
use std::{rc::Rc, cmp::Ordering};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};
use yewdux::prelude::*;

use crate::{
    api::tasks::{task::{Task, Priority}, tasks_service::TasksService},
    components::atoms::{
        button::Button,
        checkbox::Checkbox,
        dropdown::{Dropdown, DropdownOption},
        route_link::RouteLink,
    },
    router::Route,
    styles::{color::Color, styles::Styles},
    SessionStore,
};

#[derive(Clone, Copy)]
enum FilterMode {
    None,
    CompletedTasks,
    IncompletedTasks,
    PriorityA,
    PriorityB,
    PriorityC,
}

#[derive(Clone, Copy)]
enum SortMode {
    Title,
    Priority,
    Created
}

#[function_component(Tasks)]
pub fn tasks() -> Html {
    let (session_store, session_dispatch) = use_store::<SessionStore>();

    let token = match session_store.user.clone() {
        Some(user) => Some(user.token),
        None => None,
    };

    {
        let session_store = session_store.clone();
        let session_dispatch = session_dispatch.clone();
        update_tasks_in_store(session_store, session_dispatch);
    }

    let mut tasks: Vec<Task> = Vec::new();

    if let Some(new_tasks) = session_store.deref().clone().tasks {
        tasks = new_tasks;
    }

    let filter_state = use_state(|| FilterMode::None);
    let sort_state = use_state(|| SortMode::Created);

    tasks = filter_tasks(tasks, *filter_state);
    tasks = sort_tasks(tasks, *sort_state);

    let token = token.clone();
    let output = tasks.iter().map(|task| {
        let token = token.clone();
        let task = task.clone();
        let session_dispatch = session_dispatch.clone();
        let remove_onclick = delete_task_callback(task.clone(), session_dispatch.clone(), token.clone().unwrap(), || {});
        let toggle_completed = toggle_completed_callback(task.clone(), session_dispatch.clone(), token.clone().unwrap());
        html! {
            <tr>
                <td data-test={"priority"}>
                    {
                        match &task.priority {
                        Some(p) => p.to_string(),
                        None => "-".to_string()
                        }
                    }
                </td>
                <td>
                    <Checkbox data_test={"completed"} checked={task.completed()} onclick={toggle_completed}/>
                </td>
                <td>
                    <RouteLink data_test={"tasklink"} link={Route::TaskDetails { id: task.id }} text={task.title.clone()} fore_color={Color::CustomStr("black".to_string())} />
                </td>
                <td>
                    <RouteLink data_test={"delete"} link={Route::Home} onclick={remove_onclick} text={"Remove"} fore_color={Color::Error} />
                </td>
            </tr>
        }
    });
    let filter_state = filter_state.clone();

    let apply_filter = Callback::from(move |event: Event| {
        let target_element = event.target_unchecked_into::<HtmlInputElement>();
        let filter_raw = target_element.value().clone();
        let filter = match filter_raw.as_str() {
            "completed" => FilterMode::CompletedTasks,
            "incompleted" => FilterMode::IncompletedTasks,
            "priority-a" => FilterMode::PriorityA,
            "priority-b" => FilterMode::PriorityB,
            "priority-c" => FilterMode::PriorityC,
            _ => FilterMode::None,
        };
        filter_state.set(filter);
    });

    let sort_state = sort_state.clone();

    let apply_sort = Callback::from(move |event: Event| {
        let target_element = event.target_unchecked_into::<HtmlInputElement>();
        let sort_raw = target_element.value().clone();
        let sort = match sort_raw.as_str() {
            "title" => SortMode::Title,
            "priority" => SortMode::Priority,
            "created" => SortMode::Created,
            _ => SortMode::Created
        };
        sort_state.set(sort);
    });

    let history = use_history().unwrap();
    let new_task = Callback::from(move |_| {
        let history = history.clone();
        history.push(Route::NewTask);
    });

    let (style, div_style) = Styles::get_table_style();

    html! {
        <>
            <div class={div_style}>
                <Dropdown label={"Filter"} options={get_filter_options()} data_test={"filter"} selected_option={get_filter_selected_option()} onchange={apply_filter}/>
                <Dropdown label={"Sort"} options={get_sort_options()} data_test={"sort"} selected_option={get_sort_selected_option()} onchange={apply_sort}/>
                <Button label={"+ add new task"} onclick={new_task} data_test={"add-task"}/>
            </div>
            <div class={style}>
                <table>
                <col style="width:10%" />
                <col style="width:10%" />
                <col style="width:60%" />
                <col style="width:20%" />
                    <thead>
                        <th>{"Priority"}</th>
                        <th>{"Completed?"}</th>
                        <th>{"Title"}</th>
                        <th></th>
                    </thead>
                    {for output}
                </table>
            </div>
        </>
    }
}

fn sort_tasks(mut tasks: Vec<Task>, sort: SortMode) -> Vec<Task> {
    let sort = get_sort(sort);
    tasks.sort_by(sort);
    tasks
}

fn get_sort(sort: SortMode) -> impl FnMut(&Task, &Task) -> Ordering {
    match sort {
        SortMode::Title => |task_a: &Task, task_b: &Task| {
            task_a.title.to_lowercase().cmp(&task_b.title.to_lowercase())
        },
        SortMode::Priority => |task_a: &Task, task_b: &Task| {
            let a_is_none = task_a.priority.is_none();
            let b_is_none = task_b.priority.is_none();
            
            if a_is_none && b_is_none {
                return Ordering::Equal;
            }

            if a_is_none {
                return Ordering::Greater;
            }

            if b_is_none {
                return Ordering::Less;
            }
            
            task_a.priority.partial_cmp(&task_b.priority).unwrap()
        },
        SortMode::Created => |task_a: &Task, task_b: &Task| {
            task_a.id.cmp(&task_b.id)
        },
    }
}

fn get_sort_selected_option() -> DropdownOption {
    DropdownOption {
        label: Some("Creation time".to_string()),
        value: "created".to_string(),
    }
}

fn get_sort_options() -> Vec<DropdownOption> {
    vec![
        DropdownOption {
            label: Some("Title".to_string()),
            value: "title".to_string(),
        },
        DropdownOption {
            label: Some("Priority".to_string()),
            value: "priority".to_string(),
        },
        DropdownOption {
            label: Some("Creation time".to_string()),
            value: "created".to_string(),
        }
    ]
}

fn filter_tasks(tasks: Vec<Task>, filter: FilterMode) -> Vec<Task> {
    let mut filter = get_filter(filter);
    tasks.iter().filter_map(|task| filter(task)).collect()
}

fn get_filter(filter: FilterMode) -> impl FnMut(&Task) -> Option<Task> {
    match filter {
        FilterMode::None => move |task: &Task| Some(task.clone()),
        FilterMode::CompletedTasks => move |task: &Task| match task.completed_at {
            Some(_) => Some(task.clone()),
            None => None,
        },
        FilterMode::IncompletedTasks => move |task: &Task| match task.completed_at {
            Some(_) => None,
            None => Some(task.clone()),
        },
        FilterMode::PriorityA => move |task: &Task| match task.priority.clone() {
            Some(priority) => if priority == Priority::A {
                    Some(task.clone())
                }
                else {
                    None
                },
            None => None
        },
        FilterMode::PriorityB => move |task: &Task| match task.priority.clone() {
            Some(priority) => if priority == Priority::B {
                    Some(task.clone())
                }
                else {
                    None
                },
            None => None
        },
        FilterMode::PriorityC => move |task: &Task| match task.priority.clone() {
            Some(priority) => if priority == Priority::C {
                    Some(task.clone())
                }
                else {
                    None
                },
            None => None
        },
    }
}

fn get_filter_selected_option() -> DropdownOption {
    DropdownOption {
        label: Some("None".to_string()),
        value: "none".to_string(),
    }
}

fn get_filter_options() -> Vec<DropdownOption> {
    vec![
        DropdownOption {
            label: Some("None".to_string()),
            value: "none".to_string(),
        },
        DropdownOption {
            label: Some("Completed tasks".to_string()),
            value: "completed".to_string(),
        },
        DropdownOption {
            label: Some("Incompleted tasks".to_string()),
            value: "incompleted".to_string(),
        },
        DropdownOption {
            label: Some("Priority A".to_string()),
            value: "priority-a".to_string(),
        },
        DropdownOption {
            label: Some("Priority B".to_string()),
            value: "priority-b".to_string(),
        },
        DropdownOption {
            label: Some("Priority C".to_string()),
            value: "priority-c".to_string(),
        },
    ]
}

fn toggle_completed_callback(
    task: Task,
    session_dispatch: Dispatch<SessionStore>,
    token: String,
) -> Callback<MouseEvent> {
    let mut task = task.clone();
    if let None = task.completed_at {
        task.completed_at = Some(Local::now().to_string());
    } else {
        task.completed_at = None;
    }
    let session_dispatch = session_dispatch.clone();
    let token = token.clone();
    Callback::from(move |event: MouseEvent| {
        event.prevent_default(); // lets the form to update checked status
        let token = token.clone();
        let task = task.clone();
        let session_dispatch = session_dispatch.clone();
        spawn_local(async move {
            let response = TasksService::update_task(token.clone(), task.clone()).await;
            match response {
                Ok(()) => session_dispatch.reduce(|store| {
                    let mut store = store.deref().clone();
                    store.tasks_valid = false;
                    store
                }),
                Err(error) => log!(format!("task deletion failed, details: {}", error)),
            }
        })
    })
}

pub fn update_tasks_in_store(
    session_store: Rc<SessionStore>,
    session_dispatch: Dispatch<SessionStore>,
) -> () {
    {
        let session_store = session_store.clone();
        let token = match session_store.user.clone() {
            Some(user) => Some(user.token.clone()),
            None => None,
        };
        let session_dispatch = session_dispatch.clone();
        use_effect(move || {
            if !session_store.clone().tasks_valid {
                let session_dispatch = session_dispatch.clone();
                if let Some(token) = token {
                    let session_dispatch = session_dispatch.clone();
                    spawn_local(async move {
                        let response = TasksService::get_tasks(token).await;
                        if let Ok(tasks) = response {
                            session_dispatch.reduce(|store| {
                                let mut store = store.deref().clone();
                                store.tasks = Some(tasks);
                                store.tasks_valid = true;
                                store
                            });
                        }
                    });
                }
            }

            move || {
                drop(session_store);
            }
        });
    }
}

pub fn delete_task_callback<F>(
    task: Task,
    dispatch: Dispatch<SessionStore>,
    token: String,
    action: F,
) -> Callback<MouseEvent>
where
    F: Fn() + Clone + 'static,
{
    let token = token.clone();
    let action = action.clone();
    Callback::from(move |_: MouseEvent| {
        let task_id = task.id.clone();
        let dispatch = dispatch.clone();
        let token = token.clone();
        let action = action.clone();
        spawn_local(async move {
            let response = TasksService::delete_task(token.clone(), task_id).await;
            match response {
                Ok(()) => dispatch.reduce(|store| {
                    action();
                    let mut store = store.deref().clone();
                    store.tasks_valid = false;
                    store
                }),
                Err(error) => log!(format!("task deletion failed, details: {}", error)),
            }
        })
    })
}
