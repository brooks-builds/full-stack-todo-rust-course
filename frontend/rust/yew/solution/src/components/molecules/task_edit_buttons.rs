use crate::components::atoms::bb_link::{BBLink, LinkType};
use crate::router::Route;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct NavId {
    pub id: u32,
}

#[styled_component(TaskEditButtons)]
pub fn task_edit_buttons() -> Html {
    let stylesheet = css!(r#""#);

    let current_route = use_location().unwrap().route::<Route>().unwrap();
    let task_id = match current_route {
        Route::OneTask { id } => id,
        Route::EditTask { id } => id,
        _ => unreachable!(),
    };

    html! {
      <div class={stylesheet}>
        <BBLink text="Edit" data_test="edit" route={Route::OneTask { id: task_id }} link_type={LinkType::Button} />
      </div>
    }
}
