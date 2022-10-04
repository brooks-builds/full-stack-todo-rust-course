use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::auth_store::AuthStore;

#[function_component(DisplayAuth)]
pub fn display_auth() -> Html {
    let (store, _) = use_store::<AuthStore>();
    let username = format!(
        "Username: {}",
        store.username.as_deref().unwrap_or_default()
    );
    let password = format!(
        "Password: {}",
        store.password.as_deref().unwrap_or_default()
    );
    let is_authenticated = format!("Is Authenticated: {}", store.is_authenticated);

    html! {
        <div>
            <h2>{"auth data"}</h2>
            <div>{username}</div>
            <div>{password}</div>
            <div>{is_authenticated}</div>
        </div>
    }
}
