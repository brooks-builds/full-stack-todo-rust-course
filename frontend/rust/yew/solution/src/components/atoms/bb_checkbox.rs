use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub data_test: String,
    pub label: Option<String>,
    pub id: String,
    pub onchange: Callback<OnchangeData>,
    pub checked: bool,
}

#[derive(PartialEq, Clone)]
pub struct OnchangeData {
    pub selected: bool,
    pub id: String,
}

#[styled_component(BBCheckbox)]
pub fn bb_checkbox(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
      span {
        font-size: 36px;
      }

      input + label::before {
        content: '\a0';
        display: inline-block;
        width: 36px;
        height: 36px;
        border-radius: 5px;
        background: silver;
        text-indent: 4px;
        line-height: 36px;
        margin-right: 15px;
        font-size: 36px;
      }

      input:checked + label::before {
        content: '\2713';
        background: blue;
      }

      input {
        position: absolute;
        clip: rect(0, 0, 0, 0);
      }
    "#
    );
    let onchange = {
        let props_onchange = props.onchange.clone();
        let id = props.id.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .checked();
            props_onchange.emit(OnchangeData {
                selected: value,
                id: id.clone(),
            });
        })
    };

    html! {
      <div class={stylesheet}>
        if props.label.is_some() {
          <span>{props.label.as_ref().unwrap()}</span>
        }
        <input
          type="checkbox"
          id={props.id.clone()}
          data-test={props.data_test.clone()}
          checked={props.checked}
          {onchange}
        />
        <label for={props.id.clone()}></label>
      </div>
    }
}
