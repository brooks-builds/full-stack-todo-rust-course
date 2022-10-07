use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::styles::color::Color;

#[derive(Properties, PartialEq)]
pub struct ButtonProperties {
    pub label: String,
    pub onclick: Option<Callback<MouseEvent>>,
    pub data_test: Option<String>
}

#[styled_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let style_string = format!(
        r#"
        color: {secondary};
        background-color: {highlight};
        border-radius: 12px;
        text-align: center;
        border: 1px solid transparent;
        :hover {{
            background-color: {error};
        }}
    "#,
        highlight = Color::Highlight.get_css_color(),
        secondary = Color::Secondary.get_css_color(),
        error = Color::Error.get_css_color()
    );

    let style = Style::new(style_string).unwrap();

    let data_test = props.data_test.clone().unwrap_or_default();
    
    html! {
        <>
        if let Some(onclick) = props.onclick.clone() {
            <button class={style} data-test={data_test} {onclick}>{&props.label}</button>
        }
        else {
            <button class={style} data-test={data_test}>{&props.label}</button>
        }
        </>
    }
}