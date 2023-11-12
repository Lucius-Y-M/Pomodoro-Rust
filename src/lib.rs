
pub type ChrDuration = chrono::Duration;
pub type StdDuration = std::time::Duration;
pub type Instant = std::time::Instant;


mod app;
mod countdown;
pub use app::Pomodoro;