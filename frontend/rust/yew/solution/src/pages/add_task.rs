use std::ops::Deref;

use crate::api;
use crate::api::api_errors::ApiError;
use crate::components::atoms::bb_button::{BBButton, ButtonColor};
use crate::components::atoms::bb_select::{BBSelect, SelectOption};
use crate::components::atoms::bb_text_input::{BBTextInput, InputType};
use crate::components::atoms::bb_textarea::BBTextarea;
use crate::router::Route;
use crate::store::{add_task, Store};
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[styled_component(AddTask)]
pub fn component() -> Html {
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

    let (store, dispatch) = use_store::<Store>();

    let onsubmit = {
        let title = title;
        let description = description;
        let priority = priority;
        let token = store.token.clone();
        let navigator = use_navigator().unwrap();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let token = token.clone();
            let title = title.deref().clone();
            let description = description.deref().clone();
            let priority = priority.deref().clone();
            let navigator = navigator.clone();
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::create_task(&token, title, description, priority).await {
                    Ok(task_response) => {
                        add_task(dispatch, task_response.data);
                        navigator.push(&Route::Home);
                    }
                    Err(ApiError::NotAuthenticated) => {
                        navigator.push(&Route::Home);
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
        let navigator = use_navigator().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            navigator.push(&Route::Home);
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
