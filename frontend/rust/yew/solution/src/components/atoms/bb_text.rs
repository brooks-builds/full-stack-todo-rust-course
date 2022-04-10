use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub data_test: String,
}

#[styled_component(BBText)]
pub fn bb_text(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
      font-size: 36px;
    "#
    );

    html! {
      <p class={stylesheet} data-test={props.data_test.clone()}>{&props.text}</p>
    }
}
