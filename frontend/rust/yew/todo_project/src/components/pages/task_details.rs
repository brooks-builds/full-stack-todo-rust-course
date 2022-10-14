use std::rc::Rc;
use chrono::Local;
use gloo::console::log;
use lazy_static::__Deref;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::{prelude::use_store, store::Store};

use crate::{
    api::tasks::{
        task::{Priority, Task},
        tasks_service::TasksService,
    },
    components::{
        atoms::{
            button::Button,
            checkbox::Checkbox,
            dropdown::{Dropdown, DropdownOption},
            text_display::TextDisplay,
            text_input::{ControlType, TextInput},
        },
        organisms::{
            error_message::ErrorMessage,
            tasks::{delete_task_callback, update_tasks_in_store},
        },
    },
    router::Route,
    styles::{color::Color, styles::Styles},
    SessionStore,
};

#[derive(Properties, PartialEq)]
pub struct TaskDetailsProperties {
    pub task_id: i32,
}

#[derive(Store, PartialEq, Clone, Default, Debug)]
pub struct TaskStore {
    pub task: Task,
}

#[function_component(TaskDetails)]
pub fn task_details(props: &TaskDetailsProperties) -> Html {
    let (style, button_style) = Styles::get_editable_details_style();
    
    let (session_store, session_dispatch) = use_store::<SessionStore>();

    let (task_store, task_dispatch) = use_store::<TaskStore>();

    let onchange = task_dispatch.reduce_callback_with(|store, event: Event| {
        let target_element = event.target_unchecked_into::<HtmlInputElement>();
        let value = target_element.value();
        let mut store = store.deref().clone();
        match target_element.id().as_str() {
            "title" => store.task.title = value.clone(),
            "priority" => {
                store.task.priority = match value.parse() {
                    Ok(priority) => Some(priority),
                    Err(_) => None,
                }
            }
            "description" => {
                store.task.description = if value == "" {
                    None
                } else {
                    Some(value.clone())
                }
            }
            "completed" => {
                store.task.completed_at = if store.task.completed() {
                    None
                } else {
                    Some(Local::now().to_string())
                }
            }
            _ => (),
        };
        store
    });

    {
        let session_store = session_store.clone();
        let session_dispatch = session_dispatch.clone();
        update_tasks_in_store(session_store, session_dispatch);
    }

    let edit_state = use_state(|| false);
    let task = match *edit_state {
        true => Some(task_store.task.clone()),
        false => get_task_by_id(props.task_id, session_store.clone()),
    };

    if let None = task {
        return html! {
            <ErrorMessage message={"You must be logged in to view tasks"} data_test={"error"}/>
        };
    }

    let task = task.unwrap();

    let toggle_edit = {
        let edit_state = edit_state.clone();
        let task_dispatch = task_dispatch.clone();
        let task = task.clone();
        task_dispatch.reduce_callback(move |store| {
            let edit_state = edit_state.clone();
            edit_state.set(!*edit_state);
            let mut store = store.deref().clone();
            store.task = task.clone();
            store
        })
    };

    let history = use_history().unwrap();

    let save_changes = {
        let history = history.clone();
        let edit_state = edit_state.clone();
        let mut task = task_store.task.clone();
        let session_task = get_task_by_id(task.id, session_store.clone()).unwrap_or_default();
        let token = session_store.user.clone().unwrap().token;

        if task.completed() && session_task.completed() {
            task.completed_at = session_task.completed_at.clone();
        }

        let session_dispatch = session_dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            let history = history.clone();
            let edit_state = edit_state.clone();
            let session_dispatch = session_dispatch.clone();
            let token = token.clone();
            let task = task.clone();
            spawn_local(async move {
                let response = TasksService::update_task(token.clone(), task.clone()).await;
                match response {
                    Ok(()) => {
                        history.push(Route::TaskDetails {
                            id: task.id.clone(),
                        });
                        session_dispatch.reduce(|store| {
                            edit_state.set(false);
                            let mut store = store.deref().clone();
                            store.tasks_valid = false;
                            store
                        })
                    }
                    Err(error) => log!(format!("task deletion failed, details: {}", error)),
                }
            })
        })
    };

    let history = history.clone();
    let goto_home = {
        let history = history.clone();
        Callback::from(move |_| history.push(Route::Home))
    };

    let session_dispatch = session_dispatch.clone();
    let session_store = session_store.clone();
    let delete_task = {
        let task = task.clone();
        let history = history.clone();
        let session_dispatch = session_dispatch.clone();
        let session_store = session_store.clone();
        delete_task_callback(
            task.clone(),
            session_dispatch.clone(),
            session_store.user.clone().unwrap().token.clone(),
            move || history.push(Route::Home),
        )
    };

    let session_title = get_task_by_id(props.task_id, session_store.clone())
        .unwrap_or_default()
        .title;

    html! {
        <div class={style}>
            <h3>{session_title.clone()}</h3>
            <p>{"Here you can view and edit task details."}</p>
            if *edit_state {
                <TextDisplay id={"id"} label={"ID"} text={task.id.to_string()}/>
                <TextInput data_test={"editing-title"} id={"title"} label={"Title"} text={task.title.clone()} onchange={onchange.clone()}/>
                <Dropdown data_test={"editing-priority"} id={"priority"} label={"Priority"} options={get_priority_options()} selected_option={get_selected_value(task.priority)} onchange={onchange.clone()}/>
                <TextInput data_test={"editing-description"} id={"description"} label={"Description"} control_type={ControlType::Textarea} rows={3} text={task.description.clone()} onchange={onchange.clone()}/>
                <Checkbox data_test={"completed"} id={"completed"} label={"Completed?"} checked={task.completed_at.is_some()} onchange={onchange.clone()}/>
                <div class={button_style}>
                    <Button
                        label={"Discard changes"}
                        fore_color={Color::CustomStr("white".to_string())}
                        back_color={Color::Error}
                        hover_color={Color::Error2}
                        data_test={"cancel"}
                        onclick={toggle_edit.clone()}/>
                    <Button label={"Save changes"} onclick={save_changes.clone()} data_test={"submit"}/>
                </div>
            }
            else {
                <TextDisplay label={"ID"} text={task.id.to_string()}/>
                <TextDisplay data_test={"title"} label={"Title"} text={task.title.clone()}/>
                <TextDisplay
                    data_test={"priority"}
                    label={"Priority"}
                    text={match task.priority.clone() {
                        Some(p) => p.to_string(),
                        None => "-".to_string()
                    }
                }/>
                <TextDisplay data_test={"description"} label={"Description"} text={task.description.clone().unwrap_or_default()}/>
                <TextDisplay data_test={"completed"} label={"Completed at"} text={task.completed_at.clone().unwrap_or("Task not yet completed".to_string())}/>
                <div class={button_style}>
                    <Button data_test={"edit"} label={"Edit task"} onclick={toggle_edit.clone()}/>
                    <Button
                        data_test={"delete"}
                        label={"Delete task"}
                        fore_color={Color::CustomStr("white".to_string())}
                        back_color={Color::Error}
                        hover_color={Color::Error2}
                        onclick={delete_task.clone()}/>
                    <Button label={"Return to tasks"} onclick={goto_home.clone()}/>
                </div>
            }
        </div>
    }
}

fn get_task_by_id(id: i32, store: Rc<SessionStore>) -> Option<Task> {
    let tasks = store.tasks.clone();
    if let None = tasks {
        return None;
    }

    tasks.clone().unwrap().iter().find_map(|task| {
        if task.id == id {
            Some(task.clone())
        } else {
            None
        }
    })
}

pub fn get_selected_value(priority: Option<Priority>) -> DropdownOption {
    DropdownOption {
        value: match priority {
            Some(p) => p.to_string(),
            None => "-".to_string(),
        },
        label: None,
    }
}

pub fn get_priority_options() -> Vec<DropdownOption> {
    let priorities = vec![Priority::A, Priority::B, Priority::C];

    let mut result = vec![DropdownOption {
        value: "-".to_string(),
        label: None,
    }];

    for priority in priorities.iter() {
        let priority = priority.to_string();
        result.push(DropdownOption {
            value: priority.clone(),
            label: None,
        });
    }

    result
}
