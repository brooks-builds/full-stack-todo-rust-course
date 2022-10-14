use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::styles::color::Color;

#[derive(Properties, PartialEq)]
pub struct CheckboxProperties {
    pub checked: bool,
    pub label: Option<String>,
    pub onchange: Option<Callback<Event>>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub id: Option<String>,
    pub data_test: Option<String>,
}

#[styled_component(Checkbox)]
pub fn checkbox(props: &CheckboxProperties) -> Html {
    let label_style = Style::new(format!(
        r#"
        margin-right: 20px;
        color: {};
    "#,
        Color::Primary.get_css_color()
    ))
    .unwrap();

    let main_style = Style::new(
        r#"
        display: flex;
        flex-direction: row;
        align-items: center;
    "#,
    )
    .unwrap();

    let style = Style::new(format!(
        r#"
        accent-color: {color};
        background-color: {back_color};
        height: 32px;
        width: 32px;
        "#,
        color = Color::Highlight.get_css_color(),
        back_color = Color::Secondary.get_css_color()
    ))
    .unwrap();

    html! {
        <div class={main_style}>
        if let Some(label) = props.label.clone() {
            <label class={label_style}>{label}</label>
        }
            <input 
                id={props.id.clone()}
                type={"checkbox"}
                data-test={props.data_test.clone()}
                class={style}
                checked={props.checked.clone()}
                onchange={props.onchange.clone()}
                onclick={props.onclick.clone()}/>
        </div>
    }
}
