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

    let title_onchange = Callback::from(|title| {
        log!(title);
    });
    let description_onchange = Callback::from(|description| {
        log!(description);
    });
    let priority_onchange = Callback::from(|priority| {
        log!(priority);
    });
    let completed_onchange = Callback::from(|completed| {
        log!(completed);
    });

    let task = use_store::<StoreType>()
        .state()
        .map(|store| store.get_task_by_id(props.id))
        .unwrap_or_default()
        .unwrap_or_default();

    html! {
      <section class={stylesheet}>
        <form>
          <BBTextInput data_test="editing-title" label="Task Title" input_type={InputType::Text} onchange={title_onchange} value={task.title} />
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
