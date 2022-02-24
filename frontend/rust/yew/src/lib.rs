mod components;
mod router;
mod views;

use components::navbar::NavBar;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <div>
        <NavBar />
        <BrowserRouter>
          <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
      </div>
    }
}
