use crate::api;
use crate::components::atoms::bb_button::BBButton;
use crate::components::atoms::bb_link::{BBLink, LinkType};
use crate::router::Route;
use crate::store::{remove_task_by_id, Store};
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

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

    // let current_route = use_location().unwrap().route::<Route>().unwrap();
    let current_route = use_location().unwrap().query::<NavId>().unwrap();
    let task_id = current_route.id;

    let (store, dispatch) = use_store::<Store>();

    let delete_onclick = {
        let token = store.token.clone();
        let history = use_navigator().unwrap();
        Callback::from(move |_| {
            let token = token.clone();
            let dispatch = dispatch.clone();
            let history = history.clone();
            let task_id = task_id;
            wasm_bindgen_futures::spawn_local(async move {
                api::delete_task(task_id, &token).await.unwrap();
                remove_task_by_id(dispatch, task_id);
                history.push(&Route::Home);
            });
        })
    };

    html! {
      <div class={stylesheet}>
        if task_id.is_some() {
            <BBLink text="Edit" data_test="edit" route={Route::EditTask { id: task_id.unwrap() }} link_type={LinkType::Button} />
            <BBButton data_test="delete" label="Delete" onclick={delete_onclick} />
        }
        <BBLink text="Add Task" data_test="add-task" route={Route::AddTask} link_type={LinkType::Button} />
      </div>
    }
}
