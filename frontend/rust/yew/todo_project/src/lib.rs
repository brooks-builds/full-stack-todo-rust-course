use api::{auth::auth::Auth, tasks::task::Task};
use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::{styled_component, Global};
use yewdux::prelude::*;

use crate::router::{Route, switch};

mod components;
use components::organisms::navbar::Navbar;
mod router;
mod styles;
mod api;

const MAIN_STYLESHEET: &str = include_str!("main.css");

#[derive(Store, Default, PartialEq, Clone, Debug)]
pub struct SessionStore{
    user: Option<Auth>,
    tasks: Option<Vec<Task>>,
    tasks_valid: bool,
}

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <Global css={MAIN_STYLESHEET}/>
        <BrowserRouter>
            <Navbar />
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
        </>
    }
}