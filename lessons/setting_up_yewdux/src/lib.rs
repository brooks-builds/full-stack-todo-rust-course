mod counter;
mod router;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::counter::Counter;
use router::{switch, Route};

#[derive(Clone, Default)]
pub struct YewduxState {
    pub count: u32,
}

pub struct App;

impl Component for App {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxState>>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Main App"}</h1>
                <WithDispatch<Counter> />
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        }
    }
}
