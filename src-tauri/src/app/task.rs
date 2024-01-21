use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u32,
    title: String,
    description: String,
    priority: Priority,
    estimated_time: u32,
    status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, title: String, description: String, priority: Priority, estimated_time: u32) -> Task {
        Task {
            id,
            title,
            description,
            priority,
            estimated_time,
            status: TaskStatus::NotStarted,
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

    pub fn get_priority(&self) -> &String {
        &self.title
    }
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    pub fn get_estimated_time(&self) -> &u32 {
        &self.estimated_time
    }
    pub fn set_estimated_time(&mut self, estimated_time: u32) {
        self.estimated_time = estimated_time;
    }

    pub fn get_status(&self) -> &TaskStatus {
        &self.status
    }
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
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