mod api;
mod components;
mod pages;
mod router;
mod store;

use api::{api_errors::ApiError, get_tasks};
use components::molecules::error_message::ErrorMessage;
use components::organisms::navbar::Navbar;
use gloo::{console, utils::history};
use router::{switch, Route};
use store::{set_tasks, StoreType};
use yew::prelude::*;
use yew_router::{history, prelude::*};
use yewdux_functional::use_store;

#[function_component(App)]
pub fn app() -> Html {
    let token: String = use_store::<StoreType>()
        .state()
        .map(|store| store.token.clone())
        .unwrap_or_default();
    let is_loaded = use_state(|| false);
    let dispatch = use_store::<StoreType>().dispatch().clone();
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
          <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
