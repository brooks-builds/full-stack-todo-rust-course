use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::counter::CounterStore;

pub struct IncrementCount {
    pub dispatch: Dispatch<CounterStore>,
}

#[derive(Debug)]
pub enum Msg {
    State(Rc<CounterStore>),
    ButtonClicked,
}

impl Component for IncrementCount {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<CounterStore>::subscribe(ctx.link().callback(Msg::State));
        Self { dispatch }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        gloo::console::log!("updating", format!("{:?}", &msg));
        match msg {
            Msg::ButtonClicked => self.dispatch.reduce_mut(|counter_store| {
                counter_store.count += 1;
            }),
            _ => (),
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button onclick={ctx.link().callback(|_| Msg::ButtonClicked)}>{"Increment Count"}</button>
        }
    }
}
