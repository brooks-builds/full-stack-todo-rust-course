use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;

mod components;

use components::atoms::main_title::MainTitle;

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    html! {
        <div class={stylesheet}>
            <MainTitle title="Hi there!!!!!!!" />
            <p>{"more text"}</p>
        </div>
    }
}
