pub mod init;
pub mod table;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Table {
    name: String,
    location: String,
}

impl Table {
    pub fn new<T: Into<String>>(name: T, location: T) -> Table {
        Table { name: name.into(), location: location.into() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Users {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    id: i32,
    start: usize,
    duration: i32,
    table_name: String,
    owner: String,
    public: bool,
}
