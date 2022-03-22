use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    html! {
      <form>
        <TextInput name="username" />
        <CustomButton label="Submit" />
      </form>
    }
}
