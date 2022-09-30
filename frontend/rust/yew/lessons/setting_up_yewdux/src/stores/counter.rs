use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Debug)]
pub struct CounterStore {
    pub count: u32,
}
