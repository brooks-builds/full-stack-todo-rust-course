use crate::components::pages::hello::Hello;
use crate::components::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Hello => html! { <Hello /> },
    }
}
