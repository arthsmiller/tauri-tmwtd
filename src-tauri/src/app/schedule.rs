use chrono::{NaiveTime};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub id: u32,
    pub task_id: u32,
    pub start: NaiveTime,
    pub end: NaiveTime
}

impl Schedule {
    pub fn new (id: u32, task_id: u32, start: NaiveTime, end: NaiveTime) -> Schedule {
        Schedule {
            id,
            task_id,
            start,
            end
        }
    }
}