use chrono::Duration;
use gloo::timers::callback::Timeout;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::styles::color::Color;

pub const DEFAULT_TIMEOUT_MS: u32 = 10000;

#[derive(Properties, PartialEq)]
pub struct ErrorMessageProperties {
    pub message: String,
    pub data_test: Option<String>,
    pub timeout: Option<Duration>,
    pub timeout_ms: Option<u32>
}

#[styled_component(ErrorMessage)]
pub fn error_message(props: &ErrorMessageProperties) -> Html {
    let show_error = use_state(|| true);
    
    let timeout_ms = match props.timeout.clone() {
        Some(timeout) => match u32::try_from(timeout.num_milliseconds()) {
            Ok(timeout_ms) => Some(timeout_ms),
            Err(_) => None
        },
        None => None
    };

    let timeout;

    if let Some(timeout_ms) = timeout_ms {
        timeout = timeout_ms;
    }
    else {
        timeout = match props.timeout_ms.clone() {
            Some(timeout_ms) => timeout_ms,
            None => DEFAULT_TIMEOUT_MS
        }
    }

    let display_style = if *show_error {
        ""
    }
    else {
        "display: none;"
    };

    let style = Style::new(format!(
        r#"
        @keyframes show {{
            0%   {{opacity: 0.0;}}
            25%  {{opacity: 1.0;}}
            75%  {{opacity: 1.0;}}
            100% {{opacity: 0.0}}
        }}

        animation-name: show;
        animation-duration: {duration}ms;
        background-color: {error};
        padding: 5px 0px;
        color: white;
        width: 100%;
        text-align: center;
        {display}
        "#,
        error = Color::Error.get_css_color(),
        display = display_style,
        duration = timeout
    )).unwrap();

    Timeout::new(timeout, move || {
        show_error.set(false);
    }).forget();

    html! {
        <p class={style} data-test={props.data_test.clone()}>
            {&props.message}
        </p>
    }
}