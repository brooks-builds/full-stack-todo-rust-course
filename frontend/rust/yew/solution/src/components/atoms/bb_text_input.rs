use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum InputType {
    Text,
    Password,
}

impl ToString for InputType {
    fn to_string(&self) -> String {
        match self {
            InputType::Text => "text".to_owned(),
            InputType::Password => "password".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub data_test: String,
    pub label: String,
    pub placeholder: Option<String>,
    pub class: Option<String>,
    pub input_type: InputType,
    pub onchange: Callback<String>,
}

#[styled_component(BBTextInput)]
pub fn bb_text_input(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
      label {
        font-size: 36px;
      }

      input {
        font-size: 36px;
        width: 100%;
      }
    "#
    );
    let placeholder = props.placeholder.clone().unwrap_or_default();
    let id = props.label.to_lowercase().replace(" ", "-");
    let class = props.class.clone().unwrap_or_default();
    let onchange = {
        let emit_onchange = props.onchange.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            emit_onchange.emit(value);
        })
    };

    html! {
      <div class={classes!(stylesheet, class)}>
        <div>
          <label for={id.clone()}>{&props.label}</label>
        </div>
        <div>
          <input type={props.input_type.to_string()} id={id} {placeholder} data-test={props.data_test.clone()} {onchange} />
        </div>
      </div>
    }
}
