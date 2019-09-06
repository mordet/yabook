use crate::handlers;
use crate::db;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    tables: Vec<db::Table>,
}

pub async fn handle_get() -> Result<Response, handlers::Error> {
    Ok(Response {
        tables: vec![
            db::Table::new("test", "spb"),
        ],
    })
}
