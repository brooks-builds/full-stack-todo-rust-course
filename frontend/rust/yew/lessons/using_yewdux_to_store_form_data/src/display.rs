use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::YewduxStore;

pub struct DisplayForm;

impl Component for DisplayForm {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let username = &ctx.props().state().username;
        let password = &ctx.props().state().password;

        html! {
          <div>
            <h1>{"Display Form"}</h1>
            <p>{format!("Username: {}, Password: {}", username, password)}</p>
          </div>
        }
    }
}
