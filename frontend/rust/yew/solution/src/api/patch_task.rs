use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PatchTask {
    pub title: Option<String>,
    pub priority: Option<String>,
    pub description: Option<String>,
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
                    // we need to get the current date
                    Some(String::from("2022-04-20"))
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
