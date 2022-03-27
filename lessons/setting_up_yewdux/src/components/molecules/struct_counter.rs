use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::yewdux::YewduxStore;

pub enum StructCounterMessage {
    ButtonClicked(u32),
}

pub struct StructCounter {}

impl Component for StructCounter {
    type Message = StructCounterMessage;

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().state().count;
        html! {
          <div>
            <button onclick={ctx.link().callback(|_| StructCounterMessage::ButtonClicked(2))}>{"Click Me"}</button>
            <p>{"I have been clicked "}{count}{" times"}</p>
          </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StructCounterMessage::ButtonClicked(amount) => {
                YewduxStore::increment_count(ctx.props().dispatch(), 1);
                true
            }
        }
    }
}
