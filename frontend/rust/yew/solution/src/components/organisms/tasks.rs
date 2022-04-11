use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

#[styled_component(Tasks)]
pub fn tasks(props: &Props) -> Html {
    html! {
      <table>
        <th>
          <td>{"Priority"}</td>
          <td>{"Completed"}</td>
          <td>{"Task"}</td>
        </th>
      </table>
    }
}
