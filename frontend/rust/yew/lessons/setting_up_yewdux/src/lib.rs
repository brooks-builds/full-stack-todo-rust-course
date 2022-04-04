mod counter;
mod display;
mod router;
mod store;

use counter::Counter;
use display::DisplayCount;
use router::{switch, Route};
use store::{init, YewduxStore};
use yew::prelude::*;
use yew_router::prelude::*;
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
                <WithDispatch<Counter> />
                <WithDispatch<DisplayCount> />
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        }
    }
}
