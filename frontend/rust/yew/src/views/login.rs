use std::ops::Deref;

use crate::components::atom::button::{ButtonColor, ButtonWrapper};
use crate::components::atom::text_input_wrapper::{InputType, TextInputWrapper};
use crate::store::bounce::User;
use bounce::{use_atom, UseAtomHandle};
use gloo::console::__macro::JsValue;
use gloo::console::log;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize, Serializer};
use stylist::yew::styled_component;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let state = use_state(|| UserData {
        username: "".to_owned(),
        password: "".to_owned(),
    });
    let user_store = use_atom::<User>();

    html! {
      <section>
        <h1>{"Login"}</h1>
        <form onsubmit={handle_form_submit(state.clone())}>
          <div>
            <TextInputWrapper label="Username" input_type={InputType::Text} on_change={handle_change_username(state.clone())} />
          </div>
          <div>
            <TextInputWrapper label="Password" input_type={InputType::Password} on_change={handle_change_password(state.clone())} />
          </div>
          <div>
            <ButtonWrapper label="Login" color={ButtonColor::Primary} />
          </div>
        </form>
        <div>
          <p>{"username:: "}{&state.username}</p>
          <p>{"username: "}{&state.password}</p>
          <p>{"token: "}{&user_store.token}</p>
        </div>
      </section>
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct UserData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct UserDataResponse {
    id: u32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    data: UserDataResponse,
}

async fn login_to_server(user: String) {
    let response = Request::post("http://localhost:3000/api/v1/users/login")
        .body(user)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();
    let body = response.json::<ApiResponse>().await.unwrap();
    log!(serde_json::to_string(&body).unwrap());
}

fn handle_change_username(state: UseStateHandle<UserData>) -> Callback<String> {
    Callback::from(move |changed_username| {
        let mut user = state.deref().clone();
        user.username = changed_username;
        state.set(user);
    })
}

fn handle_change_password(state: UseStateHandle<UserData>) -> Callback<String> {
    Callback::from(move |password| {
        let mut user = state.deref().clone();
        user.password = password;
        state.set(user);
    })
}

fn handle_form_submit(state: UseStateHandle<UserData>) -> Callback<FocusEvent> {
    Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let stringified_user = serde_json::to_string(&*state).unwrap();
        wasm_bindgen_futures::spawn_local(login_to_server(stringified_user));
    })
}
