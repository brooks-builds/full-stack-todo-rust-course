use stylist::{css, yew::styled_component};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TextType {
    Normal,
    Title,
}

impl Default for TextType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub data_test: String,
    pub text_type: Option<TextType>,
    pub class: Option<String>,
}

#[styled_component(BBText)]
pub fn bb_text(props: &Props) -> Html {
    let text_type = props.text_type.unwrap_or_default();

    match text_type {
        TextType::Normal => normal_text(
            props.data_test.clone(),
            &props.text,
            props.class.clone().unwrap_or_default(),
        ),
        TextType::Title => title_text(
            props.data_test.clone(),
            &props.text,
            props.class.clone().unwrap_or_default(),
        ),
    }
}

fn normal_text(data_test: String, text: &str, class: String) -> Html {
    let stylesheet = css!(
        r#"
      font-size: 36px;
      "#
    );

    html! {
      <p class={classes!(stylesheet, class)} data-test={data_test}>{text}</p>
    }
}

fn title_text(data_test: String, text: &str, class: String) -> Html {
    let stylesheet = css!(
        r#"
      font-size: 72px;
    "#
    );

    html! {
      <h1 data-test={data_test} class={classes!(stylesheet, class)}>{text}</h1>
    }
}
