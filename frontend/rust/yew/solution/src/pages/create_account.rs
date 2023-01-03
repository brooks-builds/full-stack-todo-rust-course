use crate::api;
use crate::components::molecules::account_form::{AccountForm, Action, User};
use crate::router::Route;
use crate::store::login_reducer;
use crate::store::Store;
use stylist::yew::styled_component;
use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[styled_component(CreateAccount)]
pub fn create_account() -> Html {
    let stylesheet = Style::new(css!(
        r#"
          section {
            display: flex;
            justify-content: center;
          }

          section > div {
            width: 75vw;
          }
        "#
    ))
    .unwrap();

    let history = use_navigator().unwrap();
    let (_store, dispatch) = use_store::<Store>();

    let onsubmit = {
        let store_dispatch = dispatch.clone();
        Callback::from(move |user: User| {
            let history = history.clone();
            let store_dispatch = store_dispatch.clone();

            spawn_local(async move {
                let result = api::create_account(user.username, user.password).await;
                login_reducer(result, store_dispatch);
                history.push(&Route::Home);
            });
        })
    };

    html! {
      <div class={stylesheet}>
        <h1>{"Create Account"}</h1>
        <section>
          <div>
            <AccountForm {onsubmit} action={Action::CreateAccount} />
          </div>
        </section>
      </div>
    }
}
