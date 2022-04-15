use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub id: u32,
}

#[styled_component(EditTask)]
pub fn edit_task(props: &Props) -> Html {
    let stylesheet = css!(r#""#);

    html! {
      <section class={stylesheet}>
        <h1>{"edit task for id: "}{props.id}</h1>
      </section>
    }
}
