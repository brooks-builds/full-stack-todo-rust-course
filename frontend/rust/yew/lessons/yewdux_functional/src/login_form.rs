use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::{dispatch, prelude::*};

use crate::stores::auth_store::AuthStore;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let (auth_store, dispatch) = use_store::<AuthStore>();
    let username_onchange = {
        let dispatch = dispatch.clone();
        Callback::from(move |event: Event| {
            // update the store with the new username
            let username_input = event.target_unchecked_into::<HtmlInputElement>().value();
            dispatch.reduce_mut(|store| store.username = username_input);
        })
    };

    let password_onchange = {
        let dispatch = dispatch.clone();
        Callback::from(move |event: Event| {
            // update the store with the new username
            let password_input = event.target_unchecked_into::<HtmlInputElement>().value();
            dispatch.reduce_mut(|store| store.password = password_input);
        })
    };

    let onsubmit = {
        let dispatch = dispatch.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            let auth_store = dispatch.get();
            if auth_store.username.is_empty() || auth_store.password.is_empty() {
                gloo::console::error!(
                    "username or password missing, I should do something with styles here"
                );
                return;
            }
            let token = "123412341234";
            dispatch.reduce_mut(|store| store.token = Some(token.to_owned()));
        })
    };

    html! {
        <form {onsubmit}>
            <h2>{"Login"}</h2>
            <div>
                <div>
                    <label for="username">{"Username"}</label>
                </div>
                <div>
                    <input type="text" id="username" placeholder="username" onchange={username_onchange} />
                </div>
            </div>
            <div>
                <div>
                    <label for="password">{"Password"}</label>
                </div>
                <div>
                    <input type="password" id="password" placeholder="password" onchange={password_onchange} />
                </div>
            </div>
            <div>
                <button>{"Submit"}</button>
            </div>
        </form>
    }
}
