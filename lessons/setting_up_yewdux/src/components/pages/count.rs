use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::yewdux::YewduxStore;

pub enum CountMessage {
    ButtonClicked,
}

pub struct Count;

impl Component for Count {
    type Message = CountMessage;

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CountMessage::ButtonClicked => {
                log!("clicked");
                ctx.props().dispatch().reduce(|state| state.count += 1)
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <div>
            <h1>{"Count"}</h1>
            <p>{format!("The button has been clicked {} times", ctx.props().state().count)}</p>
            <div>
              <button onclick={ctx.link().callback(|_| CountMessage::ButtonClicked)}>{"Click me"}</button>
            </div>
          </div>
        }
    }
}
