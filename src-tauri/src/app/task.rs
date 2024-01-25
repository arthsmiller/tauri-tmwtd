use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
}

impl Task {
    pub fn new(id: u32, title: String, description: String, priority: Priority, estimated_time: u32) -> Task {
        Task {
            id,
            title,
            description,
        }
    }

    pub fn get_id(&self) -> &u32 {
        &self.id
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

}

pub struct TaskManager {
    tasks: Vec<Task>
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), Box<dyn Error>> {
        if self.tasks.iter().any(|t| t.get_id() == task.get_id()) {
            Err(Box::<dyn Error>::from("Task with this ID already exists"))
        } else {
            self.tasks.push(task);
            Ok(())
        }
    }
    pub fn remove_task(&mut self, task_id: u32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    pub fn find_task(&self, task_id: u32) -> Option<&Task> {
        Option::None
    }
    pub fn update_task(&mut self, task: Task) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    pub fn  list_all_tasks(&mut self) -> Vec<Task> {
        Vec::new()
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