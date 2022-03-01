use crate::components::atom::link_wrapper::LinkWrapper;
use crate::router::Route;
use stylist::yew::styled_component;
use yew::{html, Callback, Event};

#[styled_component(Home)]
pub fn home() -> Html {
    html! {
      <div>
        <LinkWrapper to_internal={Route::Task} label="home" is_button={true} />
        <LinkWrapper to="http://localhost:8080" label="vue todo" />
        <input type="text" placeholder="hello" onchange={Callback::from(|event: Event| {

          // if let Some(target) = event.target() {
          //   log!("{}", target);
          // }
        })} />
      </div>
    }
}
