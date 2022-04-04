use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::YewduxStore;

pub struct DisplayCount;

impl Component for DisplayCount {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().state().count;

        html! {
          <div>
            <h1>{"Display Count"}</h1>
            <p>{format!("Count: {}", count)}</p>
          </div>
        }
    }
}
