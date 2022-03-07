use std::fs::read_to_string;

use stylist::yew::styled_component;
use yew::prelude::*;

const STYLE: &'static str = include_str!("style.css");

#[derive(Clone, Properties, PartialEq)]
pub struct ButtonWrapperProps {
    pub label: String,
    pub color: ButtonColor,
}

#[derive(Clone, PartialEq)]
pub enum ButtonColor {
    Primary,
    Success,
    Danger,
    Warning,
    Info,
    None,
}

impl ToString for ButtonColor {
    fn to_string(&self) -> String {
        match self {
            ButtonColor::Primary => "primary".to_owned(),
            ButtonColor::Success => "success".to_owned(),
            ButtonColor::Danger => "danger".to_owned(),
            ButtonColor::Warning => "warning".to_owned(),
            ButtonColor::Info => "info".to_owned(),
            ButtonColor::None => "".to_owned(),
        }
    }
}

#[styled_component(ButtonWrapper)]
pub fn button_wrapper(props: &ButtonWrapperProps) -> Html {
    let stylesheet = stylist::Style::new(STYLE).unwrap();
    html! {
      <div class={classes!(stylesheet, css!("display: inline;"))}>
        <button class={props.color.to_string()}>{&props.label}</button>
      </div>
    }
}
