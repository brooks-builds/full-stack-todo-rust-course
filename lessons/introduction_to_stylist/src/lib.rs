use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = style!(
        r#"
            h1 {
                color: orange;
            }

            p {
                color: white;
            }
        "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
            <h1>{"Hello World!!!"}</h1>
            <p>{"more text"}</p>
        </div>
    }
}
