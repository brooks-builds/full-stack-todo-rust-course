use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::auth_store::AuthStore;

pub enum Msg {
    Store(Rc<AuthStore>),
    Username(String),
    Password(String),
    Login,
}

pub struct AuthForm {
    dispatch: Dispatch<AuthStore>,
}

impl Component for AuthForm {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<AuthStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_) => false,
            Msg::Username(username) => {
                self.dispatch
                    .reduce_mut(|store| store.username = Some(username));
                false
            }
            Msg::Password(password) => {
                self.dispatch
                    .reduce_mut(|store| store.password = Some(password));
                false
            }
            Msg::Login => {
                self.dispatch.reduce_mut(|store| {
                    store.is_authenticated = store.password.is_some() && store.username.is_some();
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            Msg::Login
        });

        let username_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let username = target.value();
            Msg::Username(username)
        });

        let password_onchange = ctx.link().callback(|event: Event| {
            let target = event.target_unchecked_into::<HtmlInputElement>();
            let password = target.value();
            Msg::Password(password)
        });

        html! {
            <form {onsubmit}>
                <h2>{"Login"}</h2>
                <div>
                    <div>
                        <label for="username">{"Username"}</label>
                    </div>
                    <div>
                        <input type="text" id="username" placeholder="username" onchange={username_onchange} />
                    </div>
                </div>
                <div>
                    <div>
                        <label for="password">{"Password"}</label>
                    </div>
                    <div>
                        <input type="password" id="password" placeholder="password" onchange={password_onchange} />
                    </div>
                </div>
                <div>
                    <button>{"Login"}</button>
                </div>
            </form>
        }
    }
}
