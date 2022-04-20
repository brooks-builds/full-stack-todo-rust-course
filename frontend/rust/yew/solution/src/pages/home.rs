use std::{ops::Deref, rc::Rc};

use crate::{
    api::{get_tasks, TaskResponse},
    components::organisms::tasks::Tasks,
    store::{self, set_tasks, Store, StoreType, Task},
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

    let tasks = use_store::<StoreType>()
        .state()
        .map(|store| store.tasks.clone())
        .unwrap_or_default();
    html! {
      <section class={stylesheet}>
        <Tasks {tasks} />
      </section>
    }
}
