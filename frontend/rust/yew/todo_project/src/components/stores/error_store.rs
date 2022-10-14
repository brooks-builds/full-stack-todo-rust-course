use uuid::Uuid;
use yewdux::store::Store;

#[derive(Store, Default, PartialEq)]
pub struct ErrorStore {
    pub message: String,
    pub display: bool,
    pub uuid: Uuid
}

impl ErrorStore {
    pub fn new(message: String, display: bool, uuid: Uuid) -> Self {
        Self {
            message,
            display,
            uuid
        }
    }
}