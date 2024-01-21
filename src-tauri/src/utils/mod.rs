pub use crate::utils::time_utils::TimeUtils;
pub use crate::utils::data_utils::DataUtils;

mod time_utils;
mod data_utils;

pub struct UtilsModule {
    time_utils: TimeUtils,
    data_utils: DataUtils,
}

impl UtilsModule {
    pub fn new() -> UtilsModule {
        UtilsModule {
            time_utils: TimeUtils::new(),
            data_utils: DataUtils::new(),
        }
    }

    // Method to configure the UtilsModule
    pub fn configure(&mut self) {
        // Utility module configuration code goes here
    }

    // Additional utility-related methods can be added here...
    // For example, methods for data conversion, time calculations, etc.
}
