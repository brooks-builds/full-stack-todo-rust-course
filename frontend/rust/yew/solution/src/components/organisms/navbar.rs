use crate::components::atoms::bb_link::{BBLink, LinkType};
use crate::router::Route;
use crate::store::Store;
use gloo::console::log;
use stylist::{css, yew::styled_component};
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

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

    let store = use_store::<PersistentStore<Store>>();
    let username = store
        .state()
        .map(|store| store.username.clone())
        .unwrap_or_default();
    let token = store
        .state()
        .map(|store| store.token.clone())
        .unwrap_or_default();
    html! {
      <section class={stylesheet}>
        <BBLink text={"Todo".to_owned()} data_test={"logo".to_owned()} route={Route::Home} />
        if token.is_empty() {
          <BBLink text={"Create Account".to_owned()} data_test={"create-account".to_owned()} route={Route::CreateAccount} link_type={LinkType::Button} />
        } else {
          <div data-test="welcome">{format!("Welcome, {}", username)}</div>
        }
      </section>
    }
}
