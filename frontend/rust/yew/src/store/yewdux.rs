use yew::Properties;
use yewdux::prelude::{BasicStore, Dispatch, Dispatcher};

#[derive(Clone, Properties, PartialEq)]
pub struct YewduxAppState {
    pub text_input: String,
}

impl Default for YewduxAppState {
    fn default() -> Self {
        Self {
            text_input: "Starting value".into(),
        }
    }
}

pub fn init() -> Dispatch<BasicStore<YewduxAppState>> {
    Dispatch::<BasicStore<YewduxAppState>>::new()
}

pub fn set_text_input(dispatch: Dispatch<BasicStore<YewduxAppState>>) -> yew::Callback<String> {
    dispatch.reduce_callback_with(|state, text_input| state.text_input = text_input)
}
