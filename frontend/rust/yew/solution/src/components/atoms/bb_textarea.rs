use std::ops::Deref;

use stylist::{yew::styled_component, Style};
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
    let stylesheet = Style::new(css!(
        r#"
        label {
          font-size: 36px;
        }

        textarea {
          font-size: 36px;
          width: 100%;
        }
      "#
    ))
    .unwrap();

    let state = use_state(String::new);
    let has_loaded = use_state(|| false);
    let onchange = {
        let props_onchange = props.onchange.clone();
        let state = state.clone();
        Callback::from(move |event: Event| {
            let change = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            props_onchange.emit(change.clone());
            state.set(change);
        })
    };

    {
        let state = state.clone();
        let has_loaded = has_loaded;
        let value = props.value.clone();
        use_effect(move || {
            if let Some(value) = value {
                if !*has_loaded && state.is_empty() {
                    state.set(value);
                    has_loaded.set(true);
                }
            }

            || {}
        })
    }

    html! {
      <div class={stylesheet}>
        <label for={props.id.clone()}>{&props.label}</label>
        <textarea
          data-test={props.data_test.clone()}
          id={props.id.clone()}
          value={state.deref().clone()}
          {onchange}
        />
      </div>
    }
}
