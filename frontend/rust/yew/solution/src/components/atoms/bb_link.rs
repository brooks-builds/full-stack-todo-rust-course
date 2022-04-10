use crate::router::Route;
use stylist::{css, yew::styled_component, StyleSource};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
pub enum LinkType {
    Link,
    Button,
}

impl Default for LinkType {
    fn default() -> Self {
        Self::Link
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub data_test: String,
    pub route: Route,
    pub link_type: Option<LinkType>,
}

#[styled_component(BBLink)]
pub fn bb_link(props: &Props) -> Html {
    let link_type = props.link_type.clone().unwrap_or_default();
    let stylesheet = choose_stylesheet(link_type);

    html! {
    <span data-test={props.data_test.clone()}>
      <Link<Route> to={props.route.clone()} classes={classes!(stylesheet)}>{props.text.clone()}</Link<Route>>
      </span>
    }
}

fn choose_stylesheet(link_type: LinkType) -> StyleSource<'static> {
    let link_stylesheet = css!(
        r#"
              color: antiquewhite;
              text-decoration: none;
              font-size: 36px;
          "#
    );
    let button_stylesheet = css!(
        r#"
          text-decoration: none;
          font-size: 32px;
          background-color: aquamarine;
          padding: 3px;
          color: black;
          border-radius: 3px;
          margin: 0 10px;
        "#
    );

    match link_type {
        LinkType::Link => link_stylesheet,
        LinkType::Button => button_stylesheet,
    }
}
