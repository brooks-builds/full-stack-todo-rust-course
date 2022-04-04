use yew::prelude::*;

pub struct StructHello {
    pub message: String,
}

impl Component for StructHello {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            message: "Hello World from a Struct!".to_owned(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <h1>{&self.message}</h1>
        }
    }
}
