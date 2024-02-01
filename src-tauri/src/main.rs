// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod utils;
mod db;
mod migrations;
mod app;


use std::error::Error;
use std::fmt::Pointer;
use std::sync::Mutex;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use crate::app::{Task, Schedule};
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
fn get_all_tasks(db_conn: tauri::State<Mutex<rusqlite::Connection>>) -> Vec<Task> {
    let conn = db_conn.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM tasks")
        .expect("Failed to prepare query.");

    let task_iter = stmt
        .query_map([], |row| {
            let id: u32 = row.get(0)?;
            let title: String = row.get(1)?;

            Ok(Task {
                id,
                title,
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

#[tauri::command]
fn get_schedule(db_conn: tauri::State<Mutex<rusqlite::Connection>>) -> Vec<Schedule> {
    let conn = db_conn.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM schedule")
        .expect("Failed to prepare query.");

    let schedule_iter = stmt
        .query_map([], |row| {
            let id: u32 = row.get(0)?;
            let task_id: u32 = row.get(1)?;
            let start: NaiveTime = row.get(2)?;
            let end: NaiveTime = row.get(3)?;

            Ok(Schedule {
                id,
                task_id,
                start,
                end,
            })
        })
        .expect("Failed to execute query.");

    let schedule: Vec<Schedule> = schedule_iter
        .filter_map(|schedule: Schedule| schedule.ok())
        .collect();

    schedule
}

#[tauri::command]
pub fn get_current_task (db_conn: tauri::State<Mutex<rusqlite::Connection>>) -> Task {
    let current_time = chrono::Local::now().format("%H:%M").to_string();

    let conn = db_conn.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM schedule WHERE start < ? AND end > ?")
        .expect("Failed to prepare query.");

    stmt
        .query_map(&[&current_time, &current_time], |row| {
            let id: u32 = row.get(0)?;
            let title: String = row.get(1)?;

            Ok(Task {
                id,
                title,
            })
        })
        .expect("Failed to execute query.")
}

#[tauri::command]
pub fn get_time_remaining(db_conn: tauri::State<Mutex<rusqlite::Connection>>) -> String {
    let current_time = chrono::Local::now().format("%H:%M").to_string();

    let conn = db_conn.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT end FROM schedule WHERE start <= ? AND end > ? LIMIT 1")
        .expect("Failed to prepare query.");

    let result = stmt
        .query_map(&[&current_time, &current_time], |row| row.get(0))
        .expect("Failed to execute query.");

    match result.into_iter().next() {
        Some(Ok(end_time)) => end_time,
        _ => "No current task found.".to_string(),
    }
}


/**
TODO these functions:
1. ⚠️ load tasks
2. ⚠️ load schedule
3. ⚠️ load current task
4. ⚠️ countdown current task
5. schedule task
6. delete task
7. delete from schedule
8. action buttons
    a. abort (stop and mark as failed)
    b. finish (stop and mark as success)
    c. pause (move end time (may conflict with the task after, edgecases))
*/

fn main() {
    let mut conn = get_database_connection();

    let migrations = rusqlite_migration::Migrations::new(migrations::migrations());
    migrations.to_latest(&mut conn).unwrap();

    tauri::Builder::default()
        .manage(Mutex::new(conn))
        .invoke_handler(tauri::generate_handler![
            greet,
            update_current_time,
            get_all_tasks,
            add_new_task,
            // TODO add functions here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
