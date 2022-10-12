use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::styles::color::Color;

#[derive(PartialEq, Clone)]
pub struct DropdownOption {
    pub value: String,
    pub label: Option<String>
}

#[derive(Properties, PartialEq)]
pub struct DropdownProperties {
    pub label: String,
    pub options: Vec<DropdownOption>,
    pub selected_option: Option<DropdownOption>,
    pub onchange: Option<Callback<Event>>,
    pub id: Option<String>,
    pub data_test: Option<String>
}

#[styled_component(Dropdown)]
pub fn dropdown(props: &DropdownProperties) -> Html {
    let select_style = format!(
        r#"
        color: {secondary};
        background-color: {info};
        border-radius: 3px;
        border: 1px solid transparent;
        border-top: none;
        border-bottom: 1px solid #DDD;
        box-shadow: inset 0 1px 2px rgba({secondary},.39), 0 -1px 1px {secondary}, 0 1px 0 {secondary};
    "#,
        secondary = Color::Secondary.get_css_color(),
        info = Color::Info.get_css_color()
    );

    let select_style = Style::new(select_style).unwrap();

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

    let options = props.clone().options.iter().map(|option|{
        let option = option.clone();
        let selected_option = props.selected_option.clone();
        html! {
            if selected_option == Some(option.clone()) {
                <option value={option.value.clone()} selected=true>
                    {option.label.clone().unwrap_or(option.value.clone())}
                </option>
            }
            else {
                <option value={option.value.clone()}>
                    {option.label.clone().unwrap_or(option.value.clone())}
                </option>
            }
        }
    });
    
    html! {
        <div class={main_style}>
            <label class={label_style}>{props.label.clone()}</label>
            <select id={props.id.clone()} class={select_style} data-test={data_test} onchange={props.onchange.clone()}>
                {for options}
            </select>
        </div>
    }
}