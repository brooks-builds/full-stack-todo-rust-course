mod api;
mod display;
mod login;
mod store;

use login::Login;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use yew::prelude::*;

#[derive(Clone)]
pub struct User {
    pub token: String,
    pub tasks: Option<Vec<Task>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub completed_at: Option<String>,
    pub description: Option<String>,
    pub id: u32,
    pub priority: char,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskResponse {
    pub data: Vec<Task>,
}

#[function_component(App)]
pub fn view() -> Html {
    let should_say_hi = use_state(|| true);

    let should_say_hi_clone = should_say_hi.clone();
    let id = gloo::timers::callback::Timeout::new(5000, move || {
        should_say_hi_clone.set(false);
    })
    .forget();

    html! {
        <div>
            <h1>{"App"}</h1>
            if *should_say_hi {
                <p>{"Hello world"}</p>
            }
        </div>
    }
}
