use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data_test: String,
    pub label: String,
}

#[styled_component(BBButton)]
pub fn bb_button(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
          width: 100%;
          font-size: 36px;
          margin: 10px 0;
          background-color: aquamarine;
        "#
    );

    html! {
      <button data-test={props.data_test.clone()} class={stylesheet}>{&props.label}</button>
    }
}
