use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub data_test: String,
    pub value: Option<String>,
    pub onchange: Callback<String>,
    pub label: String,
    pub id: String,
}

#[styled_component(BBTextarea)]
pub fn bb_textarea(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
        label {
          font-size: 36px;
        }

        textarea {
          font-size: 36px;
          width: 100%;
        }
      "#
    );

    let value = props.value.clone().unwrap_or_default();
    let onchange = {
        let props_onchange = props.onchange.clone();
        Callback::from(move |event: Event| {
            let change = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            props_onchange.emit(change);
        })
    };

    html! {
      <div class={stylesheet}>
        <label for={props.id.clone()}>{&props.label}</label>
        <textarea
          data-test={props.data_test.clone()}
          id={props.id.clone()}
          {value}
          {onchange}
        />
      </div>
    }
}
