use crate::components::atoms::bb_link::BBLink;
use crate::components::atoms::bb_text::{BBText, Color};
use crate::router::Route;
use crate::store::Task;
use stylist::yew::styled_component;
use yew::prelude::*;

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
    html! {
      <table class={stylesheet}>
        <thead>
          <th>{"Priority"}</th>
          <th>{"Completed"}</th>
          <th>{"Task"}</th>
        </thead>
        {table_data(&props.tasks)}
      </table>
    }
}

fn table_data(tasks: &Vec<Task>) -> Vec<Html> {
    let mut result = vec![];
    for task in tasks {
        let priority = task.priority.clone().unwrap_or("C".to_owned());
        result.push(html! {
          <tr>
            <td><BBText text={priority} data_test="priority" color={choose_priority_color(&priority)} /></td>
            <td></td>
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
