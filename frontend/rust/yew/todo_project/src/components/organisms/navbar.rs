use stylist::{Style, yew::styled_component};
use yew::prelude::*;

use crate::router::Route;
use crate::components::atoms::route_link::RouteLink;

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let style = Style::new(r#"
        border-bottom: 1px solid white;
        padding: 15px 25px;
    "#).unwrap();
    html! {
        <section class={style}>
            <RouteLink text={"My TODO App"} link={Route::Home} data_test={"logo"}/>
        </section>
    }
}
