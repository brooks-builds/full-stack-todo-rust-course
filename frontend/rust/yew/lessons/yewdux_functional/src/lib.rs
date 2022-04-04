mod display;
mod login;
mod store;

use display::Display;
use login::Login;
use yew::prelude::*;

#[function_component(App)]
pub fn view() -> Html {
    html! {
        <div>
            <h1>{"App"}</h1>
            <Login />
            <Display />
        </div>
    }
}
