use yew::prelude::*;
use yewdux::prelude::*;

use crate::{components::organisms::display::Display, stores::yewdux::YewduxStore};

pub struct DisplayPage;

impl Component for DisplayPage {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <WithDispatch<Display> />

        }
    }
}
