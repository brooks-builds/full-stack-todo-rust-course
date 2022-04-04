use std::any::type_name;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "Brooks";
    let my_object = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("my name is", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    let class = "my_title";
    html! {
        <>
            <h1 class={class}>{"Hello World!!!"}</h1>
            <p>{"Hi there!"}</p>
        </>
    }
}
