use crate::components::atoms::bb_text::Color;
use crate::components::molecules::error_message;
use crate::router::Route;
use crate::store::{self, Task};
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
          margin-bottom: 25px;
        }

        .row {
          display: flex;
          justify-content: center;
        }

        text-align: center;
    "#
    );

    let sent_logged_out_error = use_state(|| false);
    let task = use_store::<StoreType>()
        .state()
        .map(|store| store.get_task_by_id(props.id))
        .unwrap_or_default()
        .unwrap_or_default();

    {
        let is_logged_out = use_store::<StoreType>()
            .state()
            .map(|store| store.token.is_empty())
            .unwrap_or_default();
        let error_message = use_store::<StoreType>()
            .state()
            .map(|store| store.error_message.clone())
            .unwrap_or_default();
        let dispatch = use_store::<StoreType>().dispatch().clone();

        use_effect(move || {
            if !(*sent_logged_out_error) && is_logged_out && error_message.is_empty() {
                store::set_error_message(dispatch, "You must be logged in to view tasks");
                sent_logged_out_error.set(true);
            }
            || {}
        })
    }

    html! {
      <section class={stylesheet}>
        <div class="title">
          <BBText text_type={TextType::Title} data_test="title" text={task.title} />
        </div>
        <div class="row">
          <BBText text="Completed: " data_test="completed-text" />
          if task.completed_at.is_some() {
            <BBText text="✓" data_test="completed" />
          } else {
            <BBText text="X" data_test="completed" color={Color::Danger} />
          }
        </div>
        <div class="row">
          <BBText text="Priority: " data_test="priority-text" />
          <BBText text={task.priority.unwrap_or_default()} data_test="priority" color={Color::Info} />
          </div>
        <div class="row">
          <BBText text={task.description.unwrap_or_default()} data_test="description" />
        </div>
      </section>
    }
}
