use std::{thread, time::Duration};

use gloo::console::log;
use stylist::yew::styled_component;
use web_sys::{Event, EventTarget, InputEvent, KeyboardEvent};
use yew::{html, use_state, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct TextInputWrapperProps {}

#[styled_component(TextInputWrapper)]
pub fn text_input_wrapper(props: &TextInputWrapperProps) -> Html {
    let data = use_state(|| String::new());
    let oninput = {
        let data = data.clone();
        Callback::from(move |event: InputEvent| {
            data.set(format!("{}{}", *data, event.data().unwrap()));
            log!(event.data());
            log!(data.as_str());
        })
    };

    html! {
      <div>
        <label for="input">{"I am an input"}</label>
        <input type="text" id="input" oninput={oninput}/>
        <p>{data.as_str()}</p>
      </div>
    }
}
