use chrono::Local;
use gloo::console::log;
use lazy_static::__Deref;
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::{
    api::tasks::{task::Task, tasks_service::TasksService},
    components::{atoms::{
        button::Button,
        text_input::{ControlType, TextInput},
        dropdown::Dropdown, checkbox::Checkbox
    }, pages::task_details::{get_priority_options, get_selected_value}},
    styles::{color::Color, styles::Styles},
    SessionStore, router::Route,
};

#[derive(Store, PartialEq, Clone, Default, Debug)]
pub struct TaskStore {
    pub task: Task
}


#[styled_component(NewTask)]
pub fn new_task() -> Html {
    let (style, button_style) = Styles::get_editable_details_style();

    let history = use_history().unwrap();
    let history = history.clone();
    let goto_home = {
        let history = history.clone();
        Callback::from(move |_| history.push(Route::Home))
    };
    
    let (session_store, session_dispatch) = use_store::<SessionStore>();

    let (task_store, task_dispatch) = use_store::<TaskStore>();

    let onchange = task_dispatch.reduce_callback_with(|store, event: Event| {
        let target_element = event.target_unchecked_into::<HtmlInputElement>();
        let value = target_element.value();
        let mut store = store.deref().clone();
        match target_element.id().as_str() {
            "title" => store.task.title = value.clone(),
            "priority" => store.task.priority = match value.parse() {
                    Ok(priority) => Some(priority),
                    Err(_) => None
                },
            "description" => store.task.description = 
                if value == "" {
                    None
                }
                else {
                    Some(value.clone())
                },
            "completed" => store.task.completed_at = 
                if store.task.completed() {
                    None
                }
                else {
                    Some(Local::now().to_string())
                },
            _ => (),
        };
        store
    });
    
    let create_task = 
    {
        let history = history.clone();
        let token = match session_store.user.clone() {
            Some(user) => Some(user.token.clone()),
            None => None
        };
        let task = task_store.task.clone();

        let session_dispatch = session_dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            let history = history.clone();
            let session_dispatch = session_dispatch.clone();
            let token = token.clone();
            let task = task.clone();

            if let None = token {
                return;
            }

            spawn_local(async move {
                let response = TasksService::create_task(token.clone().unwrap(), task.clone()).await;
                match response {
                    Ok(_) => {
                        history.push(Route::Home);
                        session_dispatch.reduce(|store| {
                            let mut store = store.deref().clone();
                            store.tasks_valid = false;
                            store
                        })
                    },
                    Err(error) => log!(format!("task creation failed, details: {}", error)),
                }
            })
        })
    };

    html!{
        <div class={style}>
            <h3>{"Create new task!"}</h3>
            <TextInput data_test={"title"} id={"title"} label={"Title"} onchange={onchange.clone()}/>
            <Dropdown data_test={"priority"} id={"priority"} label={"Priority"} options={get_priority_options()} selected_option={get_selected_value(None)} onchange={onchange.clone()}/>
            <TextInput data_test={"description"} id={"description"} label={"Description"} control_type={ControlType::Textarea} rows={3} onchange={onchange.clone()}/>
            <Checkbox data_test={"completed"} id={"completed"} label={"Completed?"} checked={task_store.task.completed()} onchange={onchange.clone()}/>
            <div class={button_style}>
                <Button
                    label={"Cancel"}
                    fore_color={Color::CustomStr("white".to_string())}
                    back_color={Color::Error}
                    hover_color={Color::Error2}
                    data_test={"cancel"}
                    onclick={goto_home.clone()}/>
                <Button label={"Create task"} onclick={create_task.clone()} data_test={"submit"}/>
            </div>
        </div>
    }
}