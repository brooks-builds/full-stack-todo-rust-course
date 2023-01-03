mod api;
mod components;
mod pages;
mod router;
mod store;

use api::{api_errors::ApiError, get_tasks};
use components::molecules::error_message::ErrorMessage;
use components::organisms::navbar::Navbar;
use gloo::console;
use router::{switch, Route};
use store::{set_tasks, Store};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let token = store.token.clone();
    let is_loaded = use_state(|| false);
    use_effect(move || {
        if !token.is_empty() && !*is_loaded {
            let dispatch = dispatch.clone();
            let is_loaded = is_loaded.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // let tasks = get_tasks(&token).await;
                match get_tasks(&token).await {
                    Ok(tasks) => {
                        set_tasks(tasks, dispatch.clone());
                    }
                    Err(ApiError::NotAuthenticated) => {
                        store::logout(dispatch.clone());
                    }
                    Err(error) => {
                        console::error!(error.to_string());
                    }
                }
                is_loaded.set(true);
            });
        }

        || {}
    });

    html! {
        <BrowserRouter>
          <Navbar />
          <ErrorMessage />
          <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
