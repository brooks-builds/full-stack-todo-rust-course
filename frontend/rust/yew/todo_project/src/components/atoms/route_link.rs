use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    router::Route,
    styles::{color::Color, styles::Styles},
};

#[derive(Properties, PartialEq)]
pub struct RouteLinkProperties {
    pub text: String,
    pub link: Option<Route>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub data_test: Option<String>,
    pub fore_color: Option<Color>,
    pub back_color: Option<Color>,
    pub hover_color: Option<Color>,
}

#[styled_component(RouteLink)]
pub fn link(props: &RouteLinkProperties) -> Html {
    let classes = classes!(Styles::get_link_style(
        props.fore_color.clone(),
        props.back_color.clone(),
        props.hover_color.clone()
    ));

    html! {
        if let Some(route) = props.link.clone() {
            <a data-test={props.data_test.clone()} onclick={props.onclick.clone()}>
                <Link<Route> to={route} {classes}>{&props.text}</Link<Route>>
            </a>
        }
        else {
            <a class={classes} data-test={props.data_test.clone()} onclick={props.onclick.clone()}>
                {&props.text}
            </a>
        }
    }
}
