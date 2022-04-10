mod api;
mod components;
mod pages;
mod router;
mod store;

use components::organisms::navbar::Navbar;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
          <Navbar />
          <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
