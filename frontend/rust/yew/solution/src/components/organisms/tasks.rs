use crate::api::api_errors::ApiError;
use crate::components::atoms::bb_checkbox::{BBCheckbox, OnchangeData};
use crate::components::atoms::bb_link::BBLink;
use crate::components::atoms::bb_text::{BBText, Color};
use crate::router::Route;
use crate::store::{StoreType, Task};
use crate::{api, store};
use stylist::yew::styled_component;
use yew::prelude::*;
use yewdux_functional::use_store;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub tasks: Vec<Task>,
}

#[styled_component(Tasks)]
pub fn tasks(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
    width: 50%;
    text-align: center;
  "#
    );

    let completed_onchange = {
        let token = use_store::<StoreType>()
            .state()
            .map(|state| state.token.clone())
            .unwrap_or_default();
        let dispatch = use_store().dispatch().clone();
        Callback::from(move |data: OnchangeData| {
            let token = token.clone();
            let task_id = data.id.parse().unwrap();
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if data.selected {
                    match api::complete_task(task_id, &token).await {
                        Ok(_) => store::mark_task_completed(dispatch, task_id),
                        Err(ApiError::NotAuthenticated) => store::logout(dispatch),
                        Err(error) => {
                            gloo::console::error!("error completing task", error.to_string());
                        }
                    }
                } else {
                    match api::uncomplete_task(task_id, &token).await {
                        Ok(_) => store::mark_task_uncompleted(dispatch, task_id),
                        Err(ApiError::NotAuthenticated) => store::logout(dispatch),
                        Err(error) => {
                            gloo::console::error!("error completing task", error.to_string());
                        }
                    }
                }
            });
        })
    };

    html! {
      <table class={stylesheet}>
        <thead>
          <th>{"Priority"}</th>
          <th>{"Completed"}</th>
          <th>{"Task"}</th>
        </thead>
        {table_data(&props.tasks, completed_onchange)}
      </table>
    }
}

fn table_data(tasks: &[Task], completed_onchange: Callback<OnchangeData>) -> Vec<Html> {
    let mut result = vec![];
    for task in tasks {
        let priority = task.priority.clone().unwrap_or_else(|| "C".to_owned());
        result.push(html! {
          <tr>
            <td><BBText text={priority} data_test="priority" color={choose_priority_color(&priority)} /></td>
            <td><BBCheckbox data_test="completed" id={task.id.to_string()} onchange={completed_onchange.clone()} checked={task.completed_at.is_some()} /></td>
            <td><BBLink text={task.title.clone()} data_test={"tasklink".to_owned()} route={Route::OneTask{id: task.id}} /></td>
          </tr>
        })
    }
    result
}

fn choose_priority_color(priority: &str) -> Color {
    match priority {
        "A" => Color::Danger,
        "B" => Color::Info,
        _ => Color::Normal,
    }
}
