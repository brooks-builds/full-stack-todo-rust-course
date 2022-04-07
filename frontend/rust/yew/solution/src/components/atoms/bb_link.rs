use crate::router::Route;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub data_test: String,
    pub route: Route,
}

#[styled_component(BBLink)]
pub fn bb_link(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
            color: antiquewhite;
            text-decoration: none;
            font-size: 36px;
        "#
    );

    html! {
      <span data-test={props.data_test.clone()}>
        <Link<Route> to={props.route.clone()} classes={classes!(stylesheet)}>{props.text.clone()}</Link<Route>>
      </span>
    }
}
