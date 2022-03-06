use bounce::use_atom;
use yew::{function_component, html, Html};

use crate::components::atom::link_wrapper::LinkWrapper;
use crate::router::Route;
use crate::store::bounce::TextInput;

#[function_component(HomeWithBounce)]
pub fn home_with_bounce() -> Html {
    let text_input = use_atom::<TextInput>().clone();
    html! {
      <div>
        <LinkWrapper to_internal={Route::Home} label="Home" is_button={true} />
        <h1>{"Bounce"}</h1>
        <p>{&text_input.inner}</p>
      </div>
    }
}
