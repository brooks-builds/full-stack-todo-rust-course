use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

use crate::{api::api_login, store::YewduxStore};

#[function_component(Login)]
pub fn view() -> Html {
    let store = use_store::<PersistentStore<YewduxStore>>();
    let handle_form_submit = {
        let dispatch = store.dispatch().clone();
        store
            .dispatch()
            .reduce_callback_with(move |state, event: FocusEvent| {
                event.prevent_default();
                let username = state.username.clone();
                let password = state.password.clone();
                let dispatch = dispatch.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = api_login(username, password).await;
                    let token = response.token;
                    dispatch.reduce(move |state| state.token = token);
                })
            })
    };
    let handle_username_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let username = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.username = username;
        });

    let handle_password_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let password = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.password = password;
        });

    let token = if let Some(state) = store.state() {
        state.token.clone()
    } else {
        String::new()
    };

    html! {
      <form onsubmit={handle_form_submit}>
        <h1>{"Login"}</h1>
        <div>
            <input type="text" placeholder="username" onchange={handle_username_change} />
        </div>
        <div>
            <input type="password" placeholder="password" onchange={handle_password_change} />
        </div>
        <div>
            <button>{"Log in"}</button>
        </div>
        <div>
            <p>{format!("token: {}", token)}</p>
        </div>
      </form>
    }
}
