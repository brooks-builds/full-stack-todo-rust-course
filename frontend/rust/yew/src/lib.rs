mod components;
mod router;
mod views;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::router::{switch, Route};

#[derive(Debug, Default, Clone)]
struct AppState {
    form_input: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| AppState::default());
    let on_form_input_change = {
        Callback::from(move |value: String| {
            let mut app_state = (*state).clone();
            app_state.form_input = value;
            state.set(app_state);
        })
    };

    html! {
      <div>
        <BrowserRouter>
          <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
      </div>
    }
}
