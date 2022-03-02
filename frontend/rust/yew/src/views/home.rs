use crate::components::atom::{link_wrapper::LinkWrapper, text_input_wrapper::TextInputWrapper};
use crate::router::Route;
use gloo::console::log;
use stylist::yew::styled_component;
use yew::{html, Callback, Event};

#[styled_component(Home)]
pub fn home() -> Html {
    let on_input_change = Callback::from(|value| log!("logging from home:", value));

    html! {
      <div>
        <LinkWrapper to_internal={Route::Task} label="home" is_button={true} />
        <LinkWrapper to="http://localhost:8080" label="vue todo" />
        <TextInputWrapper {on_input_change} />
      </div>
    }
}
