use std::time::{Duration, SystemTime};
use std::fmt;

pub struct TimeUtils;

impl TimeUtils {
    // Initializes a new instance of TimeUtils
    pub fn new() -> Self {
        TimeUtils
    }

    // Configures TimeUtils; placeholder for any potential setup
    pub fn configure(&mut self) {
        // Configuration logic here, if necessary
    }

    // Calculates the duration between two SystemTime instances
    pub fn calculate_duration(start_time: SystemTime, end_time: SystemTime) -> Result<Duration, &'static str> {
        end_time.duration_since(start_time).map_err(|_| "End time is earlier than start time")
    }

    // Formats a Duration into a human-readable string
    pub fn format_duration(duration: Duration) -> String {
        let seconds = duration.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    }
}