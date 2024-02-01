use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
}

impl Task {
    pub fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Postponed,
    Completed,
    OnHold,
    Cancelled,
}