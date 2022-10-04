mod display_auth;
mod login_form;
mod store;

use display_auth::DisplayAuth;
use login_form::LoginForm;
use yew::prelude::*;

#[function_component(App)]
pub fn view() -> Html {
    html! {
        <div>
            <h1>{"App"}</h1>
            <LoginForm />
            <DisplayAuth />
        </div>
    }
}
