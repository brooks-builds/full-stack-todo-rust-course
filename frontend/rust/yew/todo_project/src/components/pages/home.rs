use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{SessionStore, styles::styles::Styles, components::organisms::tasks::Tasks};

#[function_component(Home)]
pub fn home() -> Html {
    let (store, _) = use_store::<SessionStore>();

    let style = Styles::get_home_style();
    
    html! {
        <div class={style}>
        <h2>{"Your TODO list"}</h2>
        if let Some(user) = store.user.clone() {
            <p data-test={"welcome"}>{format!("Welcome, {name}!", name = user.username)}</p>
            <p>{"Here you can add, delete and modify your tasks!"}</p>
            <Tasks />
        }
        else {
            <p>{"Here you could add, delete and modify your tasks, if you were logged in.."}</p>
        }
        </div>
    }
}
