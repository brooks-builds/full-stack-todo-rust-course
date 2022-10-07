use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{SessionStore, styles::styles::Styles};

#[function_component(Home)]
pub fn home() -> Html {
    let (store, _) = use_store::<SessionStore>();

    let style = Styles::get_home_style();

    html! {
        <div class={style}>
        <h2>{"The TODO App home"}</h2>
        if let Some(user) = store.user.clone() {
            <p data-test={"welcome"}>{format!("Welcome, {name}!", name = user.username)}</p>
        }
        </div>
    }
}
