use stylist::yew::styled_component;
use yew::prelude::*;

mod components;

use components::atoms::main_title::{Color, MainTitle};

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <MainTitle title="Hi there!!!!!!!" color={Color::Ok} />
        </div>
    }
}
