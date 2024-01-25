use std::error::Error;
use chrono::{DateTime, NaiveDate, NaiveTime};
use crate::app::task::Priority;
pub use crate::app::task::Task;
use crate::app::TaskStatus;

pub struct Schedule {
    pub id: u32,
    pub task_id: u32,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub status: TaskStatus,
    pub priority: Priority,
}

impl Schedule {

}