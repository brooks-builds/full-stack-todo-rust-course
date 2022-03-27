use yew::Callback;
use yewdux::prelude::*;

#[derive(Clone)]
pub struct YewduxStore {
    pub count: u32,
}

impl YewduxStore {
    pub fn increment_count(dispatch: &DispatchProps<BasicStore<YewduxStore>>, amount: u32) {
        dispatch.reduce(move |state| state.count += amount);
    }
}

impl Default for YewduxStore {
    fn default() -> Self {
        Self { count: 5 }
    }
}

pub fn init_yewdux() -> Dispatch<BasicStore<YewduxStore>> {
    Dispatch::new()
}
