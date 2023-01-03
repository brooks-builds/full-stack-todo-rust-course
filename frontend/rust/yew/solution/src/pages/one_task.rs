use crate::components::atoms::bb_text::Color;
use crate::store;
use crate::{
    components::atoms::bb_text::{BBText, TextType},
    store::Store,
};
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: u32,
}

#[styled_component(OneTask)]
pub fn one_task(props: &Props) -> Html {
    let stylesheet = Style::new(css!(
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
    ))
    .unwrap();

    let sent_logged_out_error = use_state(|| false);
    let (store, dispatch) = use_store::<Store>();
    let task = store.get_task_by_id(props.id).unwrap_or_default();

    {
        let is_logged_out = store.token.is_empty();
        let error_message = store.error_message.clone();
        let dispatch = dispatch.clone();

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
