use std::ops::Deref;

use crate::api;
use crate::api::api_errors::ApiError;
use crate::components::atoms::bb_button::{BBButton, ButtonColor};
use crate::components::atoms::bb_select::{BBSelect, SelectOption};
use crate::components::atoms::bb_text_input::{BBTextInput, InputType};
use crate::components::atoms::bb_textarea::BBTextarea;
use crate::router::Route;
use crate::store::{add_task, StoreType};
use gloo::utils::history;
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::history::{self, History};
use yew_router::hooks::use_history;
use yewdux::dispatch;
use yewdux_functional::use_store;

#[styled_component(AddTask)]
pub fn add_task() -> Html {
    let stylesheet = css!(
        r#"
      form > div {
        margin-top: 10px;
      }

      .submit-buttons button {
        margin-right: 10px;
      }
    "#
    );
    let title = use_state(String::new);
    let description = use_state(|| Some(String::new()));
    let priority = use_state(|| "A".to_owned());

    let priority_options = vec![
        SelectOption::new("A", "A", true),
        SelectOption::new("B", "B", false),
        SelectOption::new("C", "C", false),
    ];

    let title_onchange = {
        let title = title.clone();
        Callback::from(move |new_title| {
            title.set(new_title);
        })
    };
    let description_onchange = {
        let description = description.clone();
        Callback::from(move |new_description| {
            description.set(Some(new_description));
        })
    };
    let priority_onchange = {
        let priority = priority.clone();
        Callback::from(move |new_priority| {
            priority.set(new_priority);
        })
    };

    let onsubmit = {
        let title = title.clone();
        let description = description.clone();
        let priority = priority.clone();
        let token = use_store::<StoreType>()
            .state()
            .map(|state| state.token.clone())
            .unwrap_or_default();
        let history = use_history().unwrap();
        let dispatch = use_store().dispatch().clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            let token = token.clone();
            let title = title.deref().clone();
            let description = description.deref().clone();
            let priority = priority.deref().clone();
            let history = history.clone();
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::create_task(&token, title, description, priority).await {
                    Ok(task_response) => {
                        add_task(dispatch, task_response.data);
                        history.push(Route::Home);
                    }
                    Err(ApiError::NotAuthenticated) => {
                        history.push(Route::Home);
                    }
                    Err(error) => {
                        gloo::console::error!(error.to_string());
                        panic!();
                    }
                };
            });
        })
    };

    let cancel_onclick = {
        let history = use_history().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            history.push(Route::Home);
        })
    };

    html! {
      <section class={stylesheet}>
        <form {onsubmit}>
          <div>
            <BBTextInput
              data_test="title"
              label="Title"
              placeholder="Task Title"
              input_type={InputType::Text}
              onchange={title_onchange}
            />
          </div>
          <div>
            <BBTextarea
              data_test="description"
              onchange={description_onchange}
              label="Task Description"
              id="new-task"
            />
          </div>
          <div>
            <BBSelect
              data_test="priority"
              id="new-priority"
              label="Priority"
              options={priority_options}
              onchange={priority_onchange}
            />
          </div>
          <div class="submit-buttons">
            <BBButton
              data_test="submit"
              label="Create Task"
            />
            <BBButton
              data_test="cancel"
              label="Cancel"
              onclick={cancel_onclick}
              color={ButtonColor::Red}
            />
          </div>
        </form>
      </section>
    }
}
