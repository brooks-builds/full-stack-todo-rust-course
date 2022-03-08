use std::ops::Deref;

use crate::components::atom::button::{ButtonColor, ButtonWrapper};
use crate::components::atom::text_input_wrapper::{InputType, TextInputWrapper};
use crate::store::bounce::User;
use bounce::use_atom;
use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Login)]
pub fn login() -> Html {
    let user_store = use_atom::<User>();
    let user_store_clone = user_store.clone();
    let username_onchange = Callback::from(move |username_value: String| {
        let mut user = user_store_clone.deref().clone();
        user.username = Some(username_value);
        user_store_clone.set(user);
    });
    let user_store_clone = user_store.clone();
    let password_onchange = Callback::from(move |password: String| {
        let mut user = user_store_clone.deref().clone();
        user.password = Some(password);
        user_store_clone.set(user);
    });
    let form_onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        user_store.deref().clone().login_to_server();
    });

    html! {
      <section>
        <h1>{"Login"}</h1>
        <form onsubmit={form_onsubmit}>
          <div>
            <TextInputWrapper label="Username" on_change={username_onchange} input_type={InputType::Text} />
          </div>
          <div>
            <TextInputWrapper label="Password" on_change={password_onchange} input_type={InputType::Password} />
          </div>
          <div>
            <ButtonWrapper label="Login" color={ButtonColor::Success} />
          </div>
        </form>
      </section>
    }
}
