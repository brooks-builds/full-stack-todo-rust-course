use yew::{html, Html};
use yew_router::Routable;

use crate::views::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/task")]
    Task,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Task => html! {<div><Home /><Home /><Home /></div>},
    }
}
