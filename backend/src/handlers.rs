pub mod tables;
pub mod booking;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
