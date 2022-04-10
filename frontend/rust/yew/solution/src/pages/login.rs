use stylist::{css, yew::styled_component};
use yew::prelude::*;

#[styled_component(Login)]
pub fn login() -> Html {
    let stylesheet = css!(r#""#);

    html! {
      <h1 class={stylesheet}>{"Login"}</h1>
    }
}
