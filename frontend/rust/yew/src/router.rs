use crate::views::home::Home;
use crate::views::home_with_bounce::HomeWithBounce;
use crate::views::home_with_yewdux::HomeWithYewdux;
use yew::{html, Html};
use yew_router::Routable;
use yewdux::prelude::WithDispatch;

#[derive(Clone, Copy, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/yewdux")]
    Yewdux,
    #[at("/bounce")]
    Bounce,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Yewdux => html! {<WithDispatch<HomeWithYewdux> />},
        Route::Bounce => html! {<HomeWithBounce />},
    }
}
