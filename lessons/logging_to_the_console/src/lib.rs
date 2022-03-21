use gloo::console::log;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let name = "Brooks";
    log!("hello", 1234, name);
    html! {
        <h1>{"Hello World!!!"}</h1>
    }
}
