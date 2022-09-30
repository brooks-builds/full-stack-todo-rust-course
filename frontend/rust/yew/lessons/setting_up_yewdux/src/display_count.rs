use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::Dispatch;

use crate::stores::counter::CounterStore;

pub enum Msg {
    State(Rc<CounterStore>),
}

pub struct DisplayCount {
    counter: Rc<CounterStore>,
}

impl Component for DisplayCount {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<CounterStore>::subscribe(ctx.link().callback(Msg::State));
        Self {
            counter: dispatch.get(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.counter = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h2>
                    {"Counter Value"}
                </h2>
                <div>
                    {self.counter.count}
                </div>
            </div>
        }
    }
}
