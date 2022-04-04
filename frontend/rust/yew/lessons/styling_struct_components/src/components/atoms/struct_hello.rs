use stylist::{style, Style};
use yew::prelude::*;

pub struct StructHello {
    pub message: String,
    pub stylesheet: Style,
}

impl StructHello {
    fn style() -> Style {
        style!(
            r#"
            color: green;
        "#
        )
        .unwrap()
    }
}

impl Component for StructHello {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            message: "Hello World from a Struct!".to_owned(),
            stylesheet: Self::style(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <h1 class={self.stylesheet.clone()}>{&self.message}</h1>
        }
    }
}
