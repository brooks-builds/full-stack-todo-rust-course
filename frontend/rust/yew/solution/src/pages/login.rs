use crate::api;
use crate::components::molecules::account_form::{AccountForm, Action, User};
use crate::router::Route;
use crate::store::login_reducer;
use crate::store::Store;
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};
use yewdux::prelude::*;
use yewdux_functional::use_store;

#[styled_component(Login)]
pub fn login() -> Html {
    let stylesheet = css!(
        r#"
          section {
            display: flex;
            justify-content: center;
          }

          section > div {
            width: 75vw;
          }
        "#
    );

    let history = use_history().unwrap();
    let store = use_store::<PersistentStore<Store>>();
    let store_dispatch = store.dispatch();

    let onsubmit = {
        let store_dispatch = store_dispatch.clone();
        Callback::from(move |user: User| {
            let history = history.clone();
            let store_dispatch = store_dispatch.clone();

            spawn_local(async move {
                let result = api::login(user.username, user.password).await;
                history.push(Route::Home);
                login_reducer(result, store_dispatch);
            });
        })
    };

    html! {
      <div class={stylesheet}>
        <h1>{"Login"}</h1>
        <section>
          <div>
            <AccountForm {onsubmit} action={Action::Login} />
          </div>
        </section>
      </div>
    }
}
