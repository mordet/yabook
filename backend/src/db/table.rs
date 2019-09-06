extern crate postgres;

use postgres::{Connection, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub location: String,
}

impl Table {
    pub fn new<T: Into<String>>(name: T, location: T) -> Table {
        Table { name: name.into(), location: location.into() }
    }
}

pub fn insert_table(conn: &Connection, name: &str, location: &str) -> Result<u64>{
    conn.execute("INSERT INTO db.table VALUES ($1, $2) ON CONFLICT DO NOTHING",
                 &[&name, &location])
}

pub fn find_table(conn: &Connection, name: &str, location: &str) -> Option<Table>{
    for row in &conn.query(
        "SELECT name, location FROM db.table WHERE name = $1 AND location = $2",
        &[&name, &location]).unwrap() {
        return Some(Table { name: row.get(0) , location: row.get(1)});
    }
    return None;
}

pub fn tables(conn: &Connection) -> Vec<Table>{
    let mut result = Vec::new();
    for row in &conn.query(
        "SELECT name, location FROM db.table", &[]).unwrap() {

        result.push(Table { name: row.get(0) , location: row.get(1)});
    }
    return result;
}
