use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

use crate::store::YewduxStore;

#[function_component(Login)]
pub fn view() -> Html {
    let store = use_store::<BasicStore<YewduxStore>>();
    let handle_form_submit = store
        .dispatch()
        .reduce_callback_with(|state, event: FocusEvent| {
            event.prevent_default();
            // do a http call to a server to log in
            let token = "982wq345fpuhrs".to_owned();
            state.token = token;
        });
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
      </form>
    }
}
