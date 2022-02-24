use stylist::yew::styled_component;
use yew::{html, Html};

#[styled_component(Home)]
pub fn home() -> Html {
    html! {
      <div>
        <h1>{"Home"}</h1>
      </div>
    }
}
