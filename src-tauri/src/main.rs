// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ui;
mod utils;
mod db;
mod migrations;

use std::sync::Mutex;
use tauri::App;
pub use app::{Scheduler, TaskManager};
pub use ui::{ClockView, TaskList};
pub use utils::{DataUtils, TimeUtils};
use crate::db::get_database_connection;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hurensohn, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn update_current_time() -> String {
    let current_time = chrono::Local::now();
    current_time.format("%H:%M:%S").to_string()
}

fn main() {
    let mut conn = get_database_connection();

    let migrations = rusqlite_migration::Migrations::new(migrations::migrations());
    migrations.to_latest(&mut conn).unwrap();

    tauri::Builder::default()
        .manage(Mutex::new(conn))
        .invoke_handler(tauri::generate_handler![
            greet,
            update_current_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn setup_app_module(app: &App) {
//     let task_manager = TaskManager::new();
//     task_manager.configure();
//
//     let mut scheduler = Scheduler::new();
//     scheduler.configure();
//
//     let user_manager = UserManager::new();
//     user_manager.load_or_create_user();
//
//     app.register_module(task_manager);
//     app.register_module(scheduler);
//     app.register_module(user_manager);
//
//     // Additional setup steps can be added here...
// }
//
// fn setup_ui_module(app: &mut App) {
//     // Initialize ClockView for the analog clock interface
//     let mut clock_view = ClockView::new();
//     // Configure and add customizations to the clock view
//     clock_view.configure();
//
//     // Initialize TaskList to display tasks
//     let mut task_list = TaskList::new();
//     // Configure task list (like layout, event handlers, etc.)
//     task_list.configure();
//
//     // Register UI components (ClockView, TaskList) with the app
//     app.register_ui_component(clock_view);
//     app.register_ui_component(task_list);
//
//     // Additional UI setup steps can be added here...
// }
//
// fn setup_utils_module(app: &mut App) {
//     // Initialize TimeUtils for handling time-related functionalities
//     let mut time_utils = TimeUtils::new();
//     // Perform any necessary configuration for TimeUtils
//     time_utils.configure();
//
//     // Initialize DataUtils for data handling tasks
//     let mut data_utils = DataUtils::new();
//     // Configure DataUtils (like setting up serialization/deserialization formats)
//     data_utils.configure();
//
//     // Register utility modules (TimeUtils, DataUtils) with the app
//     app.register_util_module(time_utils);
//     app.register_util_module(data_utils);
//
//     // Additional utility setup steps can be added here...
// }
