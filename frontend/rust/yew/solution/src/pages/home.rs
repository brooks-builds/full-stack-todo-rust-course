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

    let token: String = use_store::<StoreType>()
        .state()
        .map(|store| store.token.clone())
        .unwrap_or_default();
    let is_loaded = use_state(|| false);
    let dispatch = use_store::<StoreType>().dispatch().clone();
    use_effect(move || {
        if !token.is_empty() && !*is_loaded {
            let dispatch = dispatch.clone();
            let is_loaded = is_loaded.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let tasks = get_tasks(&token).await;
                set_tasks(tasks, dispatch.clone());
                is_loaded.set(true);
            });
        }

        || {}
    });

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
