use stylist::{css, style, yew::styled_component, StyleSource};
use yew::{function_component, html};

// #[function_component(NavBar)]
#[styled_component(NavBar)]
pub fn hello_world() -> Html {
    html! {
      <div class={create_stylesheet()}>
        <div class="navbar">
          <div class="left">
            {"My Todo App"}
          </div>
          <div class="middle"></div>
          <div class="right"></div>
        </div>
      </div>
    }
}

fn create_stylesheet<'a>() -> StyleSource<'a> {
    css!(
        r#"
      .navbar {
        border-bottom: 1px solid white;
        padding: 5px;
      }
    "#
    )
}
