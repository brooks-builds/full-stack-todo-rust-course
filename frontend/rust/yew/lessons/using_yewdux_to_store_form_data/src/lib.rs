mod auth_form;
mod display_auth;
mod stores;

use auth_form::AuthForm;
use display_auth::DisplayAuth;
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"App"}</h1>
                <AuthForm />
                <DisplayAuth />
            </div>
        }
    }
}
