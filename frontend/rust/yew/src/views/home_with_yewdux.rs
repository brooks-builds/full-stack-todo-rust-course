use crate::store::yewdux::YewduxAppState;
use crate::{components::atom::link_wrapper::LinkWrapper, router::Route};
use yew::{html, Component};
use yewdux::prelude::{BasicStore, DispatchProps, WithDispatch};

pub struct HomeWithYewdux {}

impl Component for HomeWithYewdux {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxAppState>>;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, context: &yew::Context<Self>) -> yew::Html {
        let text_input = &context.props().state().text_input;

        html! {
          <div>
            <LinkWrapper to_internal={Route::Home} label="Home" is_button={true} />
            <h1>{"hello from Yewdux"}</h1>
            <p>{text_input}</p>
          </div>
        }
    }
}
