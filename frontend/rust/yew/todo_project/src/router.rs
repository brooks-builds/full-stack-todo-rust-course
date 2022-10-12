use crate::components::pages::{
    create_account::CreateAccount, home::Home, login::Login, new_task::NewTask,
    task_details::TaskDetails,
};
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
    TaskDetails { id: i32 },
    #[at("/tasks/new")]
    NewTask,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::CreateAccount => html! {<CreateAccount />},
        Route::Login => html! {<Login />},
        Route::TaskDetails { id } => html! { <TaskDetails task_id={*id}/>},
        Route::NewTask => html! { <NewTask /> },
    }
}
