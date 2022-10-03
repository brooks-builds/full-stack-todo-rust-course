mod display_count;
mod increment_count;
mod stores;

use display_count::DisplayCount;
use increment_count::IncrementCount;
use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"App"}</h1>
                <DisplayCount />
                <IncrementCount />
            </div>
        }
    }
}
