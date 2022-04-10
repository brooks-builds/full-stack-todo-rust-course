use crate::pages::{create_account::CreateAccount, home::Home, login::Login};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/create-account")]
    CreateAccount,
    #[at("/login")]
    Login,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::CreateAccount => html! { <CreateAccount /> },
        Route::Login => html! { <Login /> },
    }
}
