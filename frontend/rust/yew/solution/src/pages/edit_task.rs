use std::ops::Deref;

use crate::api::patch_task::PatchTask;
use crate::api::{self};
use crate::components::atoms::bb_button::{BBButton, ButtonColor};
use crate::components::atoms::bb_checkbox::{BBCheckbox, OnchangeData};
use crate::components::atoms::bb_select::{BBSelect, SelectOption};
use crate::components::atoms::bb_textarea::BBTextarea;
use crate::router::Route;
use crate::store::update_task_by_id;
use crate::{
    components::atoms::bb_text_input::{BBTextInput, InputType},
    store::Store,
};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

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
      
      .buttons button {
        margin-right: 10px;
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
        Callback::from(move |completed: OnchangeData| {
            completed_state.set(Some(completed.selected));
        })
    };
    let (store, dispatch) = use_store::<Store>();

    let onsubmit = {
        let title_state = title_state;
        let description_state = description_state;
        let priority_state = priority_state;
        let completed_state = completed_state.clone();
        let token = store.token.clone();
        let task_id = props.id;
        let history = use_history().unwrap();
        let dispatch = dispatch.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            let patch_task = PatchTask::new(
                title_state.deref().clone(),
                priority_state.deref().clone(),
                description_state.deref().clone(),
                *completed_state.deref(),
            );
            let token = token.clone();
            let task_id = task_id;
            let history = history.clone();
            let dispatch = dispatch.clone();

            wasm_bindgen_futures::spawn_local(async move {
                api::update_task(task_id, &token, patch_task.clone())
                    .await
                    .unwrap();
                history.push(Route::OneTask { id: task_id });
                update_task_by_id(dispatch, task_id, patch_task)
            });
        })
    };

    let task = store.get_task_by_id(props.id).unwrap_or_default();

    let cancel_onclick = {
        let history = use_history().unwrap();
        let task_id = props.id;
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            history.push(Route::OneTask { id: task_id })
        })
    };

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
            checked={is_completed(task.completed_at.as_ref(), *completed_state)}
          />
          <div class="buttons">
            <BBButton data_test="submit" label="Save" />
            <BBButton data_test="cancel" label="Cancel" onclick={cancel_onclick} color={ButtonColor::Red} />
          </div>
        </form>
      </section>
    }
}

fn create_priority_options(task: Option<String>) -> Vec<SelectOption> {
    let priority = task.unwrap_or_else(|| "A".into());
    let select_options = vec![
        SelectOption::new("A", "A", "A" == &priority),
        SelectOption::new("B", "B", "B" == &priority),
        SelectOption::new("C", "C", "C" == &priority),
    ];

    select_options
}

fn is_completed(task_completed_at: Option<&String>, state_completed: Option<bool>) -> bool {
    if let Some(completed) = state_completed {
        completed
    } else {
        task_completed_at.is_some()
    }
}
