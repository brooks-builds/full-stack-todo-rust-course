use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct LinkProperties {
    pub text: String,
    pub link: Route,
    pub data_test: Option<String>
}

#[styled_component(RouteLink)]
pub fn link(properties: &LinkProperties) -> Html {
    let classes = classes! (Style::new("color: white; text-decoration: none;").unwrap());
    html! {
        <a data-test={properties.data_test.clone().unwrap_or(properties.text.clone())}>
            <Link<Route> to={properties.link.clone()} {classes}>{&properties.text}</Link<Route>>
        </a>
    }
}