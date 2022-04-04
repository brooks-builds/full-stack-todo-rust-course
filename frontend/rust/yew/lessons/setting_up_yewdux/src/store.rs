use yewdux::prelude::*;

#[derive(Clone)]
pub struct YewduxStore {
    pub count: u32,
}

impl Default for YewduxStore {
    fn default() -> Self {
        Self { count: 5 }
    }
}

pub fn init() -> Dispatch<BasicStore<YewduxStore>> {
    Dispatch::<BasicStore<YewduxStore>>::new()
}
