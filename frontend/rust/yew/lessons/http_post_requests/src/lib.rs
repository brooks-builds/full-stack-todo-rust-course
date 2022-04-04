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
    let state = use_state(|| {
        User {
        token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImJyb29rcyIsImlhdCI6MTY0ODgyMDYyMH0.Go-LDpLWcUg9utWC6vQJ-jiLuW_lkaxbt6qjXwuYrxE".to_owned(),
        tasks: None
    }
    });

    let onclick = {
        let state = state.clone();
    };
    html! {
        <div>
            <h1>{"App"}</h1>
            <Login />
        </div>
    }
}
