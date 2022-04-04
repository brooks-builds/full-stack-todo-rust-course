mod display;
mod login;
mod store;

use display::DisplayForm;
use login::Login;
use store::{init, YewduxStore};
use yew::prelude::*;
use yewdux::prelude::*;

pub struct App {
    _dispatch: Dispatch<BasicStore<YewduxStore>>,
}

impl Component for App {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(_ctx: &Context<Self>) -> Self {
        let _dispatch = init();
        Self { _dispatch }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"App"}</h1>
                <WithDispatch<Login> />
                <WithDispatch<DisplayForm> />
            </div>
        }
    }
}
