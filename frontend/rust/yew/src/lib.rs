mod components;
mod router;
mod views;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <div>
      <BrowserRouter>
          <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
      </div>
    }
}
