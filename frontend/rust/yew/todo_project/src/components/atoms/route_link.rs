use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{router::Route, styles::color::Color};

#[derive(Properties, PartialEq)]
pub struct LinkProperties {
    pub text: String,
    pub link: Route,
    pub data_test: Option<String>,
    pub fore_color: Option<Color>,
    pub back_color: Option<Color>,
    pub hover_color: Option<Color>
}

#[styled_component(RouteLink)]
pub fn link(props: &LinkProperties) -> Html {
    let fore_color = match props.fore_color.clone() {
        Some(color) => color.clone().get_css_color(),
        None => Color::Primary.get_css_color()
    };

    let hover_color = match props.hover_color.clone() {
        Some(color) => color.clone().get_css_color(),
        None => Color::Info.get_css_color()
    };

    let mut style_string = format!(
        r#"
        padding: 0 0 0 15px;
        text-decoration: none;
        color: {};
        :hover {{
            color: {};
        }}
        "#,
        fore_color, hover_color);

    if let Some(color) = props.back_color.clone() {
        style_string = format!("{}background-color: {};", style_string, color.clone().get_css_color());
    }

    let classes = classes!(Style::new(style_string).unwrap());

    html! {
        <a data-test={props.data_test.clone()}>
            <Link<Route> to={props.link.clone()} {classes}>{&props.text}</Link<Route>>
        </a>
    }
}
