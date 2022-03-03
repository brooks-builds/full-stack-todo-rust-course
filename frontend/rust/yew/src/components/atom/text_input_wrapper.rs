use gloo::console::{error, log};
use std::{thread, time::Duration};
use stylist::yew::styled_component;
use web_sys::{
    Event, EventTarget, HtmlElement, HtmlInputElement, HtmlStyleElement, HtmlTextAreaElement,
    InputEvent, KeyboardEvent,
};
use yew::{classes, html, use_state, Callback, Properties};
use yew::{use_context, TargetCast};

#[derive(Properties, PartialEq, Clone)]
pub struct TextInputWrapperProps {
    pub on_change: Callback<String>,
    pub label: String,
    pub placeholder: Option<String>,
}

#[styled_component(TextInputWrapper)]
pub fn text_input_wrapper(props: &TextInputWrapperProps) -> Html {
    let onchange = {
        let cloned_props = props.clone();
        Callback::from(move |event: Event| {
            if let Some(input_element) = event.target_dyn_into::<HtmlInputElement>() {
                let value = input_element.value();
                cloned_props.on_change.emit(value);
            } else {
                error!("Error converting html input into an HtmlInputElement");
            }
        })
    };
    let stylesheet = css!(
        r#"
            display: flex;
            flex-direction: column;
            margin: 10px;

            input {
                font-size: 50px;
                padding: 5px;
                background-color: lightblue;
            }
        "#
    );

    html! {
      <div class={stylesheet}>
        <label for="input">{props.label.clone()}</label>
        <input type="text" id="input" placeholder={props.placeholder.clone().unwrap_or_else(|| props.label.clone())} onchange={onchange}/>
      </div>
    }
}
