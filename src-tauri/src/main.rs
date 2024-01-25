// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ui;
mod utils;
mod db;
mod migrations;

use std::error::Error;
use std::fmt::Pointer;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::App;
pub use app::{Schedule, TaskManager};
pub use ui::{ClockView, TaskList};
pub use utils::{DataUtils, TimeUtils};
use crate::app::Task;
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

#[tauri::command]
fn list_all_tasks(db_conn: tauri::State<Mutex<rusqlite::Connection>>) -> Vec<Task> {
    let conn = db_conn.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM tasks")
        .expect("Failed to prepare query.");

    let task_iter = stmt
        .query_map([], |row| {
            let id: u32 = row.get(0)?;
            let title: String = row.get(1)?;
            let description: String = row.get(2)?;

            Ok(Task {
                id,
                title,
                description,
            })
        })
        .expect("Failed to execute query.");

    let tasks: Vec<Task> = task_iter
        .filter_map(|task| task.ok())
        .collect();

    tasks
}

#[tauri::command]
fn add_new_task(
    title: String,
    description: String,
    db_conn: tauri::State<Mutex<rusqlite::Connection>>,
) -> Result<(), String> {
    let conn = db_conn.lock().unwrap();

    let mut stmt = conn
        .prepare("INSERT INTO tasks (title, description) VALUES (?, ?)")
        .expect("Failed to prepare query.");

    let result = stmt.execute(&[&title, &description]);

    match result {
        Ok(_) => {
            Ok(())
        }
        Err(err) => {
            Err(format!("Error inserting task: {:?}", err))
        }
    }
}

fn main() {
    let mut conn = get_database_connection();

    let migrations = rusqlite_migration::Migrations::new(migrations::migrations());
    migrations.to_latest(&mut conn).unwrap();

    tauri::Builder::default()
        .manage(Mutex::new(conn))
        .invoke_handler(tauri::generate_handler![
            greet,
            update_current_time,
            list_all_tasks,
            add_new_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
