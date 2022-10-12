use std::rc::Rc;
use chrono::Local;
use gloo::console::log;
use lazy_static::__Deref;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};
use yewdux::prelude::*;

use crate::{
    api::tasks::{task::Task, tasks_service::TasksService},
    components::atoms::{button::Button, checkbox::Checkbox, route_link::RouteLink},
    router::Route,
    styles::{color::Color, styles::Styles},
    SessionStore,
};

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
                    <Checkbox data_test={"completed"} enabled={false} checked={task.completed()} onclick={toggle_completed}/>
                </td>
                <td>
                    <RouteLink data_test={"tasklink"} link={Route::TaskDetails { id: task.id }} text={task.title.clone()} fore_color={Color::Custom("black".to_string())} />
                </td>
                <td>
                    <RouteLink data_test={"delete"} link={Route::Home} onclick={remove_onclick} text={"Remove"} fore_color={Color::Error} />
                </td>
            </tr>
        }
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

fn toggle_completed_callback(task: Task, session_dispatch: Dispatch<SessionStore>, token: String) -> Callback<MouseEvent> {
    let mut task = task.clone();
    if let None = task.completed_at {
        task.completed_at = Some(Local::now().to_string());
    }
    else {
        task.completed_at = None;
    }
    let session_dispatch = session_dispatch.clone();
    let token = token.clone();
    Callback::from(move |_| {
        let token = token.clone();
        let task = task.clone();
        let session_dispatch = session_dispatch.clone();
        spawn_local(async move {
            let response = TasksService::update_task(token.clone(), task.clone()).await;
                match response {
                    Ok(()) => {
                        session_dispatch.reduce(|store| {
                            let mut store = store.deref().clone();
                            store.tasks_valid = false;
                            store
                        })
                    },
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
    F: Fn() + Clone + 'static
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
