use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let username_state = use_state(|| "no username set".to_owned());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username| {
        cloned_username_state.set(username);
    });
    html! {
      <form>
        <TextInput name="username" handle_onchange={username_changed} />
        <CustomButton label="Submit" />
        <p>{"Username: "}{&*username_state}</p>
      </form>
    }
}
