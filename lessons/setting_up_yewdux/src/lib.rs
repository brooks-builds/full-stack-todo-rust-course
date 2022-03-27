mod components;
mod router;
mod stores;

use crate::components::molecules::custom_form::Data;
use crate::router::{switch, Route};
use crate::stores::yewdux::init_yewdux;
use components::atoms::main_title::{Color, MainTitle};
use components::atoms::struct_hello::StructHello;
use components::molecules::custom_form::CustomForm;
use components::molecules::struct_counter::StructCounter;
use gloo::console::log;
use std::ops::Deref;
use stores::yewdux::YewduxStore;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let state = init_yewdux();

    html! {
        <div>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </div>
    }
}
