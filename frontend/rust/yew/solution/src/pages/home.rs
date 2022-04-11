use std::ops::Deref;

use crate::{
    api::{get_tasks, TaskResponse},
    components::organisms::tasks::Tasks,
    store::{self, set_tasks, Store},
};
use gloo::console::log;
use reqwasm::http::Request;
use stylist::yew::styled_component;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

#[styled_component(Home)]
pub fn home() -> Hmtl {
    let stylesheet = css!(
        r#"
          display: flex;
          flex-direction: column;
          align-items: center;
        "#
    );

    let store = use_store::<PersistentStore<Store>>();
    set_tasks(store.dispatch().deref().clone());
    html! {
      <section class={stylesheet}>
        <h1>{"Home"}</h1>
        <Tasks />
      </section>
    }
}
