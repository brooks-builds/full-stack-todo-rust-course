use yewdux::prelude::*;

#[derive(Store, Default, PartialEq)]
pub struct CounterStore {
    pub count: u32,
}
