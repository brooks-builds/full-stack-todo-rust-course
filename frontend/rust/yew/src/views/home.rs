use crate::components::atom::button::{ButtonColor, ButtonWrapper};
use crate::components::atom::text_input_wrapper::InputType;
use crate::components::atom::{link_wrapper::LinkWrapper, text_input_wrapper::TextInputWrapper};
use crate::router::Route;
use crate::store::bounce::TextInput;
use crate::store::yewdux::{set_text_input, YewduxAppState};
use crate::Store;
use bounce::use_atom;
use gloo::console::log;
use stylist::yew::styled_component;
use yew::{html, use_context, use_state, Callback, Event};
use yew_agent::Dispatcher;
use yewdux::prelude::*;

#[styled_component(Home)]
pub fn home() -> Html {
    let context = use_context::<Store>().unwrap();
    let set_text_input_reducer = set_text_input(context.dispatch);
    let bounce_text_input = use_atom::<TextInput>().clone();
    let on_input_change = Callback::from(move |value: String| {
        set_text_input_reducer.emit(value.clone());
        bounce_text_input.set(TextInput { inner: value });
    });

    html! {
      <div>
        <LinkWrapper to_internal={Route::Yewdux} label="Yewdux" is_button={true} />
        <LinkWrapper to_internal={Route::Bounce} label="Bounce" is_button={true} />
        <LinkWrapper to="http://localhost:8080" label="vue todo!!!" />
        <TextInputWrapper on_change={on_input_change} label="form input" placeholder="I am a form input" input_type={InputType::Text} />
        <h1>{"button"}</h1>
        <ButtonWrapper label="primary" color={ButtonColor::Primary} />
        <ButtonWrapper label="success" color={ButtonColor::Success} />
        <ButtonWrapper label="danger" color={ButtonColor::Danger} />
        <ButtonWrapper label="warning" color={ButtonColor::Warning} />
        <ButtonWrapper label="info" color={ButtonColor::Info} />
        <ButtonWrapper label="none" color={ButtonColor::None} />
      </div>
    }
}
