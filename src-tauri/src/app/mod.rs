use std::error::Error;
pub use crate::app::task::{Task, TaskManager, TaskStatus};
pub use crate::app::scheduler::Scheduler;

mod task;
mod scheduler;

pub struct AppModule {
    task_manager: TaskManager,
    scheduler: Scheduler,
}

impl AppModule {
    pub fn new() -> AppModule {
        AppModule {
            task_manager: TaskManager::new(),
            scheduler: Scheduler::new(),
        }
    }

    // Function to configure the AppModule
    pub fn configure(&mut self) {
        // Configure the Task Manager
        // This might involve setting default parameters, loading settings from a file, etc.
        // self.task_manager.configure();

        // Configure the Scheduler
        // Set default scheduling parameters or load from a configuration file
        // self.scheduler.configure();

        // Configure the User Manager
        // Load user settings, default profiles, etc.
        // self.user_manager.configure();

        // Additional configuration steps for AppModule
        // This can include setting up application-wide settings, initializing
        // any shared resources, setting default values, etc.

        // Example: Setting up a global logger
        // setup_global_logger();

        // Example: Loading global application settings
        // load_global_settings();
    }

    // Task-related functions
    pub fn add_task(&mut self, task: task::Task) -> Result<(), Box<dyn Error>> {
        match self.task_manager.add_task(task) {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn remove_task(&mut self, task_id: u32) -> Result<(), Box<dyn Error>> {
        // Attempt to remove the task with the given ID from the Task Manager
        match self.task_manager.remove_task(task_id) {
            Ok(_) => {
                // If successful, return Ok
                Ok(())
            },
            Err(e) => {
                // If there's an error, return the error
                Err(e)
            }
        }
    }

    pub fn update_task(&mut self, task: task::Task) -> Result<(), Box<dyn Error>> {
        // Attempt to update the existing task in the Task Manager
        match self.task_manager.update_task(task) {
            Ok(_) => {
                // If successful, return Ok
                Ok(())
            },
            Err(e) => {
                // If there's an error, return the error
                Err(e)
            }
        }
    }

    pub fn list_tasks(&mut self) -> Vec<Task> {
        self.task_manager.list_all_tasks()
    }

    pub fn schedule_tasks(&mut self) -> Result<(), Box<dyn Error>> {
        let tasks = self.task_manager.list_all_tasks();

        // Schedule the tasks using the Scheduler
        match self.scheduler.schedule(&tasks) {
            Ok(scheduled_tasks) => {
                // Update the tasks in the Task Manager with their scheduled times
                for task in scheduled_tasks {
                    self.task_manager.update_task(task)?;
                }
                Ok(())
            },
            Err(e) => {
                // If there's an error in scheduling, return the error
                Err(e)
            }
        }
    }
}
