use yewdux::prelude::*;

#[derive(Clone)]
pub struct State {
    pub count: u32,
}

impl State {
    pub fn increment_count(dispatch: &DispatchProps<BasicStore<State>>, amount: u32) {
        dispatch.reduce(move |state| state.count += amount);
    }
}

impl Default for State {
    fn default() -> Self {
        Self { count: 5 }
    }
}
