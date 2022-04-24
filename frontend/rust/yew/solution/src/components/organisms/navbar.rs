use std::ops::Deref;

use crate::components::atoms::bb_link::{BBLink, LinkType};
use crate::components::atoms::bb_text::BBText;
use crate::components::molecules::task_edit_buttons::TaskEditButtons;
use crate::router::Route;
use crate::store::Store;
use serde::{Deserialize, Serialize};
use stylist::{css, yew::styled_component};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::{use_store, StoreRef};

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let stylesheet = css!(
        r#"
            border-bottom: 1px solid antiquewhite;
            padding: 10px 20px;
            display: flex;
            justify-content: space-between;
        "#
    );

    let current_route = use_location().unwrap().route::<Route>().unwrap();
    let store = use_store::<PersistentStore<Store>>();
    let (username, token) = get_from_store(store);

    html! {
      <section class={stylesheet}>
        <BBLink text={"Todo".to_owned()} data_test={"logo".to_owned()} route={Route::Home} />
        if is_logged_in(&token) {
          <div>
            <TaskEditButtons />
          </div>
        }
        if !is_logged_in(&token) {
          <div>
            <BBLink text={"Create Account".to_owned()} data_test={"create-account".to_owned()} route={Route::CreateAccount} link_type={LinkType::Button} />
            <BBLink text={"Login".to_owned()} data_test={"login".to_owned()} route={Route::Login} link_type={LinkType::Button} />
          </div>
        } else {
          <BBText data_test="welcome" text={format!("Welcome, {}", username)} />
        }
      </section>
    }
}

fn is_logged_in(token: &str) -> bool {
    !token.deref().is_empty()
}

fn get_from_store(store: StoreRef<PersistentStore<Store>>) -> (String, String) {
    let username = store
        .state()
        .map(|store| store.username.clone())
        .unwrap_or_default();
    let token = store
        .state()
        .map(|store| store.token.clone())
        .unwrap_or_default();

    (username, token)
}
