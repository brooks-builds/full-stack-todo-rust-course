use std::ops::Deref;

use crate::components::atoms::bb_button::BBButton;
use crate::components::atoms::bb_checkbox::BBCheckbox;
use crate::components::atoms::bb_select::{BBSelect, SelectOption};
use crate::components::atoms::bb_textarea::BBTextarea;
use crate::router::Route;
use crate::store::Task;
use crate::{
    components::atoms::bb_text_input::{BBTextInput, InputType},
    store::StoreType,
};
use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::{history, prelude::*};
use yewdux_functional::use_store;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub id: u32,
}

#[styled_component(EditTask)]
pub fn edit_task(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
      form > * {
        margin-top: 15px;
      }
    "#
    );

    let title_state = use_state(|| None);
    let description_state = use_state(|| None);
    let priority_state = use_state(|| None);
    let completed_state = use_state(|| None);

    let title_onchange = {
        let title_state = title_state.clone();
        Callback::from(move |title: String| {
            title_state.set(Some(title));
        })
    };
    let description_onchange = {
        let description_state = description_state.clone();
        Callback::from(move |description| {
            description_state.set(Some(description));
        })
    };
    let priority_onchange = {
        let priority_state = priority_state.clone();
        Callback::from(move |priority| {
            priority_state.set(Some(priority));
        })
    };
    let completed_onchange = {
        let completed_state = completed_state.clone();
        Callback::from(move |completed| {
            completed_state.set(Some(completed));
        })
    };
    let onsubmit = {
        let title_state = title_state.clone();
        let description_state = description_state.clone();
        let priority_state = priority_state.clone();
        let completed_state = completed_state.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            log!(
                title_state.deref().clone().unwrap_or_default(),
                description_state.deref().clone().unwrap_or_default(),
                priority_state.deref().clone().unwrap_or_default(),
                completed_state.deref().clone().unwrap_or_default()
            );
        })
    };

    let task = use_store::<StoreType>()
        .state()
        .map(|store| store.get_task_by_id(props.id))
        .unwrap_or_default()
        .unwrap_or_default();

    html! {
      <section class={stylesheet}>
        <form {onsubmit}>
          <BBTextInput
            data_test="editing-title"
            label="Task Title"
            input_type={InputType::Text}
            onchange={title_onchange}
            value={task.title.clone()}
          />
          <BBTextarea
            data_test="editing-description"
            value={task.description}
            onchange={description_onchange}
            label="Edit Description"
            id={format!("description-{}", props.id)}
          />
          <BBSelect
            data_test="editing-priority"
            id={format!("priority-{}", props.id)}
            label="Priority"
            options={create_priority_options(task.priority)}
            onchange={priority_onchange}
          />
          <BBCheckbox
            data_test="completed"
            label="Completed: "
            id={format!("completed-{}", props.id)}
            onchange={completed_onchange}
            checked={task.completed_at.is_some()}
          />
          <div>
            <BBButton data_test="submit" label="Save" />
          </div>
        </form>
      </section>
    }
}

fn create_priority_options(task: Option<String>) -> Vec<SelectOption> {
    let priority = task.unwrap_or("A".into());
    let select_options = vec![
        SelectOption::new("A", "A", "A" == &priority),
        SelectOption::new("B", "B", "B" == &priority),
        SelectOption::new("C", "C", "C" == &priority),
    ];

    select_options
}
