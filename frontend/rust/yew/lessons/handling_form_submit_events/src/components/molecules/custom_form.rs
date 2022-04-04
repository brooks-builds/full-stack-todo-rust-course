use std::clone;
use std::ops::Deref;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub favorite_language: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let language_changed = Callback::from(move |language| {
        let mut data = cloned_state.deref().clone();
        data.favorite_language = language;
        cloned_state.set(data);
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    html! {
      <form onsubmit={onsubmit}>
        <TextInput name="username" handle_onchange={username_changed} />
        <TextInput name="favorite_language" handle_onchange={language_changed} />
        <CustomButton label="Submit" />
      </form>
    }
}
