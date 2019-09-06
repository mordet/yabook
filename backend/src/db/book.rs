extern crate postgres;

use chrono::{DateTime, Utc};
use postgres::{Connection, Result};
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Book {
    id: u64,
    start: DateTime<Utc>,
    duration: Duration,
    table_name: String,
    owner: String,
    public: bool,
}
