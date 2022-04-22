use js_sys::{Date, JsString};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PatchTask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

impl PatchTask {
    pub fn new(
        title: Option<String>,
        priority: Option<String>,
        description: Option<String>,
        completed_at: Option<bool>,
    ) -> Self {
        let completed_at = match completed_at {
            Some(completed) => {
                if completed {
                    let now = Date::new_0();
                    now.to_utc_string().to_string().as_string()
                } else {
                    None
                }
            }
            None => None,
        };

        Self {
            title,
            priority,
            description,
            completed_at,
        }
    }
}
