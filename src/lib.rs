pub use std::sync::{Arc, Mutex};



#[allow(dead_code)]

pub type ChrDuration = chrono::Duration;
pub type StdDuration = std::time::Duration;
pub type Instant = std::time::Instant;

pub type ChrLocal = chrono::Local;
pub type ChrDateTime<T> = chrono::DateTime<T>;


pub type ArMut<T> = Arc<Mutex<T>>;

mod juche;
mod app;
mod countdown;
mod filesys;

pub use app::Pomodoro;