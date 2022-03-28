use super::counter::Counter;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Counter,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Counter => html! { <WithDispatch<Counter> />},
    }
}
