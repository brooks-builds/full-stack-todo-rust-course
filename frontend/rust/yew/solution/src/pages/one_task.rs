use crate::router::Route;
use crate::{
    components::atoms::bb_text::{BBText, TextType},
    store::StoreType,
};
use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux_functional::use_store;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: u32,
}

#[styled_component(OneTask)]
pub fn one_task(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
      .title {
        text-align: center;
      }
    "#
    );

    let task = use_store::<StoreType>()
        .state()
        .map(|store| store.get_task_by_id(props.id))
        .unwrap_or_default();

    let history = use_history().unwrap();

    let task = match task {
        Some(task) => task,
        None => {
            history.push(Route::Home);
            return html! {<></>};
        }
    };

    html! {
      <section class={stylesheet}>
        <BBText text_type={TextType::Title} data_test="title" text="This is the title" class="title" />
      </section>
    }
}
