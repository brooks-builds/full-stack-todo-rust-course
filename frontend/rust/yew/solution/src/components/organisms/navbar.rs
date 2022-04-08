use crate::components::atoms::bb_link::{BBLink, LinkType};
use crate::router::Route;
use stylist::{css, yew::styled_component};
use yew::prelude::*;

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
    html! {
      <section class={stylesheet}>
        <BBLink text={"Todo".to_owned()} data_test={"logo".to_owned()} route={Route::Home} />
        <BBLink text={"Create Account".to_owned()} data_test={"create-account".to_owned()} route={Route::CreateAccount} link_type={LinkType::Button} />
      </section>
    }
}
