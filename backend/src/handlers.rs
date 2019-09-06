pub mod tables;
pub mod booking;
pub mod user;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
