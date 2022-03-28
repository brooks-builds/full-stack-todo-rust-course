use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::YewduxState;

pub enum CounterMessage {
    ButtonClicked,
}

pub struct Counter {
    dispatch: DispatchProps<BasicStore<YewduxState>>,
}

impl Component for Counter {
    type Message = CounterMessage;

    type Properties = DispatchProps<BasicStore<YewduxState>>;

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = ctx.props().dispatch().clone();
        Self { dispatch }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().state().count;
        // let onclick = ctx.link().callback(|_| CounterMessage::ButtonClicked);
        let onclick = self.dispatch.reduce_callback(|state| state.count += 2);

        html! {
            <div>
                <h1>{"Counter"}</h1>
                <p>{format!("The button has been pressed {} times", count)}</p>
                <button {onclick}>{"Click me"}</button>
            </div>
        }
    }
}
