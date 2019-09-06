use crate::handlers;
use crate::db;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    tables: Vec<db::table::Table>,
}

pub async fn handle_get() -> Result<Response, handlers::Error> {
    Ok(Response {
        tables: vec![
            db::table::Table::new("test", "spb"),
        ],
    })
}
