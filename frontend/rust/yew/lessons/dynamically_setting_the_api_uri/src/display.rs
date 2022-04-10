use yew::{function_component, html};
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::store::YewduxStore;

#[function_component(Display)]
pub fn view() -> Html {
    let store = use_store::<PersistentStore<YewduxStore>>();
    let username = store
        .state()
        .map(|state| state.username.clone())
        .unwrap_or_default();
    let password = store
        .state()
        .map(|state| state.password.clone())
        .unwrap_or_default();
    let token = store
        .state()
        .map(|state| state.token.clone())
        .unwrap_or_default();

    html! {
      <div>
        <h1>{"Display Form"}</h1>
        <p>{format!("Username: {}, Password: {}", username, password)}</p>
        <p>{format!("token: {}", token)}</p>
      </div>
    }
}
