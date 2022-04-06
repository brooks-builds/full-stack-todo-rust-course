use stylist::{css, yew::styled_component};
use yew::prelude::*;

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let stylesheet = css!(
        r#"
            border-bottom: 1px solid antiquewhite;
            padding: 10px 20px;
        "#
    );
    html! {
      <section class={stylesheet}>
        {"navbar"}
      </section>
    }
}
