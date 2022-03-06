mod components;
mod router;
mod store;
mod views;

use crate::router::{switch, Route};
use bounce::BounceRoot;
use store::yewdux::{init, YewduxAppState};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use yewdux::prelude::{BasicStore, Dispatch};

#[derive(Clone, PartialEq)]
struct Store {
    pub dispatch: Dispatch<BasicStore<YewduxAppState>>,
}

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
    let store = Store { dispatch: init() };

    html! {
      <div>
        <ContextProvider<Store> context={store}>
          <BounceRoot>
            <BrowserRouter>
              <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
          </BounceRoot>
        </ContextProvider<Store>>
      </div>
    }
}
