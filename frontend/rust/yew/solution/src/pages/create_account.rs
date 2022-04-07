use crate::components::atoms::bb_text_input::{BBTextInput, InputType};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(CreateAccount)]
pub fn create_account() -> Html {
    let stylesheet = css!(
        r#"
          section {
            display: flex;
            justify-content: center;
            flex-direction: column;
          }

          .input {
            width: 75%;
          }
        "#
    );
    html! {
      <div class={stylesheet}>
        <h1>{"Create Account"}</h1>
        <section>
          <BBTextInput data_test="username" label="Username" placeholder="What username do you want?" class="input" input_type={InputType::Text} />
          <BBTextInput data_test="password" label="Password" placeholder="What is your password?" class="input" input_type={InputType::Password} />
        </section>
      </div>
    }
}
