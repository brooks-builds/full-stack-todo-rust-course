use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    html! {
      <h1>{&props.title}</h1>
    }
}
