use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;

mod components;

use components::atoms::main_title::{Color, MainTitle};
use components::molecules::custom_form::CustomForm;

use crate::components::molecules::custom_form::Data;

#[styled_component(App)]
pub fn app() -> Html {
    let main_title_load = Callback::from(|message: String| log!(message));
    let custom_form_submit = Callback::from(|data: Data| {
        log!("username is", data.username);
        log!("favorite language is", data.favorite_language);
    });
    html! {
        <div>
            <MainTitle title="Hi there!!!!!!!" color={Color::Ok} on_load={main_title_load} />
            <CustomForm onsubmit={custom_form_submit} />
        </div>
    }
}
