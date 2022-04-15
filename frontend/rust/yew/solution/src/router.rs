use crate::pages::edit_task::EditTask;
use crate::pages::one_task::OneTask;
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
    #[at("/tasks/:id")]
    OneTask { id: u32 },
    #[at("/tasks/:id/edit")]
    EditTask { id: u32 },
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::CreateAccount => html! { <CreateAccount /> },
        Route::Login => html! { <Login /> },
        Route::OneTask { id } => html! { <OneTask id={*id} /> },
        Route::EditTask { id } => html! { <EditTask id={*id} />},
    }
}
