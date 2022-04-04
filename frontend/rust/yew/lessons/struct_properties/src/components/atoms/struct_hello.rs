use stylist::{style, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: String,
}

pub struct StructHello {
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

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stylesheet: Self::style(),
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
          <h1 class={self.stylesheet.clone()}>{&context.props().message}</h1>
        }
    }
}
