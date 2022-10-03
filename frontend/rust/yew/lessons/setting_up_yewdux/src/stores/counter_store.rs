use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct CounterStore {
    pub count: u32,
}
