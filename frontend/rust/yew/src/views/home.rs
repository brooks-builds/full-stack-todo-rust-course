use crate::components::atom::{link_wrapper::LinkWrapper, text_input_wrapper::TextInputWrapper};
use crate::router::Route;
use gloo::console::log;
use stylist::yew::styled_component;
use yew::{html, use_state, Callback, Event};
use yew_agent::Dispatcher;

#[styled_component(Home)]
pub fn home() -> Html {
    let on_input_change = Callback::from(|value| log!("logging from home:", value));

    html! {
      <div>
        <LinkWrapper to_internal={Route::Task} label="home" is_button={true} />
        <LinkWrapper to="http://localhost:8080" label="vue todo" />
        <TextInputWrapper on_change={on_input_change} label="form input" placeholder="I am a form input" />
      </div>
    }
}
