use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::auth_store::AuthStore;

#[function_component(AuthView)]
pub fn auth_view() -> Html {
    let (store, _dispatch) = use_store::<AuthStore>();
    html! {
        <div>
            <h2>{"Auth Info"}</h2>
            <div>
                <div>
                    {"username:"}
                    <span>{&store.username}</span>
                </div>
                <div>
                    {"password:"}
                    <span>{&store.password}</span>
                </div>
                <div>
                    {"token:"}
                    <span>{store.token.as_deref().unwrap_or_default()}</span>
                </div>
            </div>
        </div>
    }
}
