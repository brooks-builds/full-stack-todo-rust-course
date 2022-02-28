use crate::Route;
use stylist::{css, yew::styled_component};
use yew::{classes, html, Properties};
use yew_router::components::Link;

#[derive(Properties, PartialEq)]
pub struct LinkWrapperProps {
    pub label: String,
    pub to: Option<String>,
    pub to_internal: Option<Route>,
    pub is_button: Option<bool>,
}

#[styled_component(LinkWrapper)]
pub fn link_wrapper(props: &LinkWrapperProps) -> Html {
    let stylesheet = css!(
        r#"
        a {
          color: #add8e6;
          text-decoration: none;
          margin: 5px;
        }

      .button {
        background-color: green;
        color: white;
        padding: 3px 5px;
        border-radius: 10px;
      }

      .button:hover {
        color: blue;
      }

      .button:active {
        outline: 2px solid red;
      }
  "#
    );

    let mut other_classes = vec![];

    if let Some(true) = props.is_button {
        other_classes.push("button");
    }

    if let Some(to_internal) = props.to_internal {
        html! {
          <div class={classes!(stylesheet)}>
            <Link<Route> to={to_internal} classes={classes!(other_classes)} >{&props.label}</Link<Route>>
          </div>
        }
    } else if let Some(to) = &props.to {
        html! {
          <div class={classes!(stylesheet)}>
            <a href={to.clone()} class={classes!(other_classes)}>{&props.label}</a>
          </div>
        }
    } else {
        panic!("missing to or internal route property");
    }
}
