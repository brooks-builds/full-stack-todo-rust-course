mod auth_view;
mod login_form;
mod stores;

use auth_view::AuthView;
use login_form::LoginForm;
use yew::prelude::*;

#[function_component(App)]
pub fn view() -> Html {
    html! {
        <div>
            <h1>{"App"}</h1>
            <LoginForm />
            <AuthView />
        </div>
    }
}
