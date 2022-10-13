use gloo::timers::callback::Timeout;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::styles::color::Color;

#[derive(Properties, PartialEq)]
pub struct ErrorMessageProperties {
    pub message: String,
    pub data_test: Option<String>
}

#[styled_component(ErrorMessage)]
pub fn error_message(props: &ErrorMessageProperties) -> Html {
    let show_error = use_state(|| true);

    let display_style = if *show_error {
        ""
    }
    else {
        "display: none;"
    };

    let style = Style::new(format!(
        r#"
        color: white;
        background-color: {error};
        border: 1px solid;
        border-color: {error2};
        width: 100%;
        text-align: center;
        {display}
        "#,
        error = Color::Error.get_css_color(),
        error2 = Color::Error2.get_css_color(),
        display = display_style
    )).unwrap();


    // let error_handle = show_error.clone();
    Timeout::new(30000, move || {
        show_error.set(false);
    }).forget();

    html! {
        <p class={style} data-test={props.data_test.clone()}>
            {&props.message}
        </p>
    }
}