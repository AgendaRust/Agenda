// Type definitions
pub mod note;
pub mod task;
pub mod reminder;
pub mod goal;
pub mod report;
// Re-export commonly used types
pub use task::{Task, TaskDuration};
pub use report::{StatsYearResponse, StatsMonthResponse, StatsWeekResponse};
