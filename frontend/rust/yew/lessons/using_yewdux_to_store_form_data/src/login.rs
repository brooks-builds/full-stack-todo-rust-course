use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::YewduxStore;

pub struct Login {
    _dispatch: DispatchProps<BasicStore<YewduxStore>>,
}

impl Component for Login {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(ctx: &Context<Self>) -> Self {
        let _dispatch = ctx.props().dispatch().clone();
        Self { _dispatch }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let handle_form_submit = {
            let state = ctx.props().state().clone();
            Callback::from(move |event: FocusEvent| {
                event.prevent_default();
                let username = state.username.clone();
                let password = state.password.clone();
                log!("Username: ", username, password);
            })
        };
        let handle_username_change =
            ctx.props()
                .dispatch()
                .reduce_callback_with(|state, event: Event| {
                    let username = event
                        .target()
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>()
                        .value();
                    state.username = username;
                });
        let handle_password_change =
            ctx.props()
                .dispatch()
                .reduce_callback_with(|state, event: Event| {
                    let password = event
                        .target()
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>()
                        .value();
                    state.password = password;
                });
        html! {
          <form onsubmit={handle_form_submit}>
            <h1>{"Login"}</h1>
            <div>
                <input type="text" placeholder="username" onchange={handle_username_change} />
            </div>
            <div>
                <input type="password" placeholder="password" onchange={handle_password_change} />
            </div>
            <div>
                <button>{"Log in"}</button>
            </div>
          </form>
        }
    }
}
