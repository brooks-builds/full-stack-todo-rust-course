use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::styles::color::Color;

#[derive(Properties, PartialEq)]
pub struct TextDisplayProperties {
    pub label: String,
    pub text: String,
    pub id: Option<String>,
    pub data_test: Option<String>,
}

#[styled_component(TextDisplay)]
pub fn text_input(props: &TextDisplayProperties) -> Html {
    let input_style = format!(
        r#"
        color: {info};
    "#,
        info = Color::Info.get_css_color()
    );

    let input_style = Style::new(input_style).unwrap();

    let label_style = format!(
        r#"
        margin-bottom: 10px;
        color: {};
    "#,
        Color::Primary.get_css_color()
    );

    let label_style = Style::new(label_style).unwrap();

    let main_style = Style::new(
        r#"
        display: flex;
        flex-direction: column;
    "#,
    )
    .unwrap();

    let data_test = props.data_test.clone().unwrap_or_default();

    html! {
        <div class={main_style}>
            <label class={label_style}>{&props.label}</label>
            <p class={input_style} id={props.id.clone()} data-test={data_test}>{props.text.clone()}</p>
        </div>
    }
}
