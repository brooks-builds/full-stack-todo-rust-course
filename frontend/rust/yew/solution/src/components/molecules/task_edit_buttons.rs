use crate::api;
use crate::components::atoms::bb_button::BBButton;
use crate::components::atoms::bb_link::{BBLink, LinkType};
use crate::components::atoms::bb_textarea::BBTextarea;
use crate::router::Route;
use crate::store::{remove_task_by_id, StoreType};
use gloo::utils::history;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::{history, prelude::*};
use yewdux_functional::use_store;

#[derive(Serialize, Deserialize)]
pub struct NavId {
    pub id: u32,
}

#[styled_component(TaskEditButtons)]
pub fn task_edit_buttons() -> Html {
    let stylesheet = css!(
        r#"
      display: flex;
      align-items: center;
      justify-content: center;
      height: 45px;
    "#
    );

    let current_route = use_location().unwrap().route::<Route>().unwrap();
    let task_id = match current_route {
        Route::OneTask { id } => id,
        Route::EditTask { id } => id,
        _ => unreachable!(),
    };

    let delete_onclick = {
        let token = use_store::<StoreType>()
            .state()
            .map(|state| state.token.clone())
            .unwrap_or_default();
        let dispatch = use_store::<StoreType>().dispatch().clone();
        let history = use_history().unwrap();
        Callback::from(move |_| {
            let token = token.clone();
            let dispatch = dispatch.clone();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                api::delete_task(task_id, &token).await.unwrap();
                remove_task_by_id(dispatch, task_id);
                history.push(Route::Home);
            });
        })
    };

    html! {
      <div class={stylesheet}>
        <BBLink text="Edit" data_test="edit" route={Route::EditTask { id: task_id }} link_type={LinkType::Button} />
        <BBButton data_test="delete" label="Delete" onclick={delete_onclick} />
      </div>
    }
}
