use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, Style};

use crate::router::{Route, switch};

mod components;
use components::organisms::navbar::Navbar;
mod router;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}