mod components;

use std::ops::Deref;

use crate::components::molecules::custom_form::Data;
use components::atoms::main_title::{Color, MainTitle};
use components::molecules::custom_form::CustomForm;
use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::ContextProvider;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let user_state = use_state(User::default);
    let main_title_load = Callback::from(|message: String| log!(message));
    let first_load = use_state(|| true);

    use_effect(move || {
        // this code will run on
        // - first render
        // - all re-renders
        // if auth token exists and if its our first render
        // get all the users todo tasks
        if *first_load {
            // this is only run when the component loads for the first time
            // this shouldn't run every time we refresh the state and re-render
            // do whatever data load we need to do
            first_load.set(false);
        }

        || {}
    });

    let custom_form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data: Data| {
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.favorite_language = data.favorite_language;
            user_state.set(user);
        })
    };

    html! {
        <ContextProvider<User> context={user_state.deref().clone()}>
            <MainTitle title="Hi there!!!!!!!" color={Color::Ok} on_load={main_title_load} />
            <CustomForm onsubmit={custom_form_submit} />
        </ContextProvider<User>>
    }
}
