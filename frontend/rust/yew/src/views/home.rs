use crate::components::atom::link_wrapper::LinkWrapper;
use crate::router::Route;
use stylist::yew::styled_component;
use yew::{html, Html};
use yew_router::prelude::*;

#[styled_component(Home)]
pub fn home() -> Html {
    html! {
      <div>
        <LinkWrapper to_internal={Route::Task} label="home" />
        <LinkWrapper to="http://localhost:8080" label="vue todo" />
      </div>
    }
}
