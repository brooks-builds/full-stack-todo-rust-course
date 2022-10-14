use gloo::utils::document;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::styles::color::Color;

#[derive(PartialEq, Clone)]
pub enum ControlType {
    Input,
    Textarea
}

#[derive(Properties, PartialEq)]
pub struct TextInputProperties {
    pub label: String,
    pub text: Option<String>,
    pub control_type: Option<ControlType>,
    pub id: Option<String>,
    pub placeholder: Option<String>,
    pub input_type: Option<String>,
    pub onchange: Option<Callback<Event>>,
    pub rows: Option<i32>,
    pub data_test: Option<String>,
}

#[styled_component(TextInput)]
pub fn text_input(props: &TextInputProperties) -> Html {
    let input_style = format!(
        r#"
        ::placeholder {{ /* Chrome, Firefox, Opera, Safari 10.1+ */
            color: {light_gray};
            font-style: italic;
            opacity: 0.5; /* Firefox */
        }}
        
        :-ms-input-placeholder {{ /* Internet Explorer 10-11 */
            color: {light_gray};
            font-style: italic;
        }}
        
        ::-ms-input-placeholder {{ /* Microsoft Edge */
            color: {light_gray};
            font-style: italic;
        }}

        color: {secondary};
        background-color: {info};
        border-radius: 3px;
        border: 1px solid transparent;
        border-top: none;
        border-bottom: 1px solid #DDD;
        box-shadow: inset 0 1px 2px rgba({secondary},.39), 0 -1px 1px {secondary}, 0 1px 0 {secondary};
    "#,
        light_gray = Color::CustomStr("#DDD".to_owned()).get_css_color(),
        secondary = Color::Secondary.get_css_color(),
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

    let placeholder = props.placeholder.clone().unwrap_or_default();
    let input_type = props.input_type.clone().unwrap_or("text".to_owned());
    let data_test = props.data_test.clone().unwrap_or_default();
    let control_type = props.control_type.clone().unwrap_or(ControlType::Input);
    let rows = props.rows.clone().unwrap_or(1);
    let id = props.id.clone().unwrap_or_default();

    {
        let id = id.clone();
        let text = props.text.clone().unwrap_or_default();
        let control_type = control_type.clone();
            use_effect(move || {
            if control_type == ControlType::Textarea && id != "" {
                let element = document().get_element_by_id(&id).unwrap();
                element.set_inner_html(&text);
            };

            || {}
        });
    }

    html! {
        <div class={main_style}>
            <label class={label_style}>{&props.label}</label>
            if control_type == ControlType::Input {
                <input class={input_style} id={id.clone()} {placeholder} type={input_type} onchange={props.onchange.clone()} data-test={data_test} value={props.text.clone()}/>
            }
            else{
                <textarea class={input_style} id={id.clone()} onchange={props.onchange.clone()} data-test={data_test} rows={rows.to_string()}>
                </textarea>
            }
        </div>
    }
}
