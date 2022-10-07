use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use uuid::Uuid;

use crate::styles::color::Color;

#[derive(Properties, PartialEq)]
pub struct TextInputProperties {
    pub label: String,
    pub id: Option<String>,
    pub placeholder: Option<String>,
    pub input_type: Option<String>,
    pub onchange: Option<Callback<Event>>,
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
        light_gray = Color::Custom("#DDD".to_owned()).get_css_color(),
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

    let id = props.id.clone().unwrap_or(Uuid::new_v4().as_simple().to_string());
    let placeholder = props.placeholder.clone().unwrap_or_default();
    let input_type = props.input_type.clone().unwrap_or("text".to_owned());
    let data_test = props.data_test.clone().unwrap_or_default();

    html! {
        <div class={main_style}>
            <label class={label_style}>{&props.label}</label>
            if let Some(onchange) = props.onchange.clone() {
                <input class={input_style} {id} {placeholder} type={input_type} {onchange} data-test={data_test}/>
            }
            else {
                <input class={input_style} {id} {placeholder} type={input_type} data-test={data_test}/>
            }
        </div>
    }
}
