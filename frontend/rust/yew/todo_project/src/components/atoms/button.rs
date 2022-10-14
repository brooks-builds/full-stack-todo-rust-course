use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::styles::color::Color;

#[derive(Properties, PartialEq)]
pub struct ButtonProperties {
    pub label: String,
    pub fore_color: Option<Color>,
    pub back_color: Option<Color>,
    pub hover_color: Option<Color>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub data_test: Option<String>
}

#[styled_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let color = props.fore_color.clone().unwrap_or(Color::Secondary);
    let background_color = props.back_color.clone().unwrap_or(Color::Highlight);
    let hover_color = props.hover_color.clone().unwrap_or(Color::Highlight2);

    let style_string = format!(
        r#"
        color: {color};
        background-color: {background_color};
        border-radius: 12px;
        text-align: center;
        border: 1px solid transparent;
        padding: 10px;
        cursor: pointer;
        :hover {{
            background-color: {hover_color};
        }}
    "#,
        color = color.get_css_color(),
        background_color = background_color.get_css_color(),
        hover_color = hover_color.get_css_color()
    );

    let style = Style::new(style_string).unwrap();

    let data_test = props.data_test.clone().unwrap_or_default();
    
    html! {
        <button class={style} data-test={data_test} onclick={props.onclick.clone()}>
            {&props.label}
        </button>
    }
}