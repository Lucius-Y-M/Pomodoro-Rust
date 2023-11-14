

#[allow(dead_code)]

pub type ChrDuration = chrono::Duration;
pub type StdDuration = std::time::Duration;
pub type Instant = std::time::Instant;

pub type ChrLocal = chrono::Local;
pub type ChrDateTime<T> = chrono::DateTime<T>;


mod app;
mod countdown;
pub use app::Pomodoro;