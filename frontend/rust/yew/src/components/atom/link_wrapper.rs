use crate::Route;
use eyre::{bail, Result};
use stylist::{css, yew::styled_component};
use yew::{classes, html, Html, Properties};
use yew_router::components::Link;

#[derive(Properties, PartialEq)]
pub struct LinkWrapperProps {
    pub label: String,
    pub to: Option<String>,
    pub to_internal: Option<Route>,
}

#[styled_component(LinkWrapper)]
pub fn link_wrapper(props: &LinkWrapperProps) -> Html {
    let stylesheet = css!(
        r#"
      color: #add8e6;
      text-decoration: none;
      margin: 5px;
  "#
    );

    if let Some(to_internal) = props.to_internal {
        html! {
          <Link<Route> to={to_internal} classes={classes!(stylesheet, "meow")}>{&props.label}</Link<Route>>
        }
    } else if let Some(to) = &props.to {
        html! {
          <a href={to.clone()} class={stylesheet}>{&props.label}</a>
        }
    } else {
        panic!("missing to or internal route property");
    }
}
