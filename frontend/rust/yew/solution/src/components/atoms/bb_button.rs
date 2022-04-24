use gloo::console;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data_test: String,
    pub label: String,
    pub onclick: Option<Callback<()>>,
}

#[styled_component(BBButton)]
pub fn bb_button(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
          button {
            font-size: 32px;
            background-color: aquamarine;
            padding: 1px;
            border-radius: 3px;
            border: none;
          }

          button:hover {
            cursor: pointer;
          }
        "#
    );

    let onclick = {
        let props_onclick = props.onclick.clone();
        Callback::from(move |_| {
            if let Some(props_onclick) = props_onclick.clone() {
                props_onclick.emit(());
            }
        })
    };

    html! {
      <span class={stylesheet}>
        <button data-test={props.data_test.clone()} {onclick}>{&props.label}</button>
      </span>
    }
}
