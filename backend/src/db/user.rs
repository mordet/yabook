extern crate postgres;

use postgres::{Transaction};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
}

pub fn insert_user(conn: &mut Transaction, name: &str) -> Result<u64, postgres::Error>{
    conn.execute("INSERT INTO db.user VALUES ($1) ON CONFLICT DO NOTHING",
                 &[&name])
}

pub fn find_user(conn: &mut Transaction, name: &str) -> Option<User>{
    for row in &conn.query("SELECT name FROM db.user WHERE name = $1", &[&name]).unwrap() {
        return Some(User { name: row.get(0) });
    }
    return None;
}
