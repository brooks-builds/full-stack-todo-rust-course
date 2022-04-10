use std::ops::Deref;

use crate::api;
use crate::router::Route;
use crate::store::login_reducer;
use crate::{
    components::atoms::{
        bb_button::BBButton,
        bb_text_input::{BBTextInput, InputType},
    },
    store::Store,
};
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};
use yewdux::prelude::*;
use yewdux_functional::use_store;

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub data: User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub token: String,
}

#[styled_component(CreateAccount)]
pub fn create_account() -> Html {
    let stylesheet = css!(
        r#"
          section {
            display: flex;
            justify-content: center;
          }

          section > div {
            width: 75vw;
          }
        "#
    );

    let username_state = use_state(String::default);
    let password_state = use_state(String::default);
    let history = use_history().unwrap();
    let store = use_store::<PersistentStore<Store>>();
    let store_dispatch = store.dispatch();

    let onsubmit = {
        let username_state = username_state.clone();
        let password_state = password_state.clone();
        let store_dispatch = store_dispatch.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            let username_state = username_state.clone();
            let password_state = password_state.clone();
            let history = history.clone();
            let store_dispatch = store_dispatch.clone();

            spawn_local(async move {
                let result = api::create_account(
                    username_state.deref().to_owned(),
                    password_state.deref().to_owned(),
                )
                .await;
                history.push(Route::Home);
                login_reducer(result, store_dispatch);
            });
        })
    };

    let username_onchange = Callback::from(move |username: String| {
        username_state.set(username);
    });

    let password_onchange = Callback::from(move |password: String| {
        password_state.set(password);
    });

    html! {
      <div class={stylesheet}>
        <h1>{"Create Account"}</h1>
        <section>
          <div>
            <form {onsubmit}>
              <BBTextInput data_test="username" label="Username" placeholder="What username do you want?" class="input" input_type={InputType::Text} onchange={username_onchange} />
              <BBTextInput data_test="password" label="Password" placeholder="What is your password?" class="input" input_type={InputType::Password} onchange={password_onchange} />
              <BBButton label="Create Account" data_test="submit" />
            </form>
          </div>
        </section>
      </div>
    }
}
