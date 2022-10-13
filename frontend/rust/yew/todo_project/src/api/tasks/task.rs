use std::{fmt::Display, str::FromStr};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd)]
pub enum Priority {
    A,
    B,
    C
}

impl FromStr for Priority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            _ => Err(())
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub priority: Option<Priority>,
    pub description: Option<String>,
    pub completed_at: Option<String>
}

impl Task {
    pub fn completed(&self) -> bool {
        self.completed_at.is_some()
    }
}

