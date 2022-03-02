use gloo::console::{error, log};
use std::{thread, time::Duration};
use stylist::yew::styled_component;
use web_sys::{
    Event, EventTarget, HtmlElement, HtmlInputElement, HtmlStyleElement, HtmlTextAreaElement,
    InputEvent, KeyboardEvent,
};
use yew::{html, use_state, Callback, Properties};
use yew::{use_context, TargetCast};

#[derive(Properties, PartialEq, Clone)]
pub struct TextInputWrapperProps {
    pub on_input_change: Callback<String>,
}

#[styled_component(TextInputWrapper)]
pub fn text_input_wrapper(props: &TextInputWrapperProps) -> Html {
    let data = use_state(|| String::new());
    let onchange = {
        let data = data.clone();
        let cloned_props = props.clone();
        Callback::from(move |event: Event| {
            if let Some(input_element) = event.target_dyn_into::<HtmlInputElement>() {
                let value = input_element.value();
                log!(value.clone());
                cloned_props.on_input_change.emit(value.clone());
                data.set(value);
            } else {
                error!("Error converting html input into an HtmlInputElement");
            }
        })
    };

    html! {
      <div>
        <label for="input">{"I am an input"}</label>
        <input type="text" id="input" onchange={onchange}/>
        <p>{data.as_str()}</p>
      </div>
    }
}
