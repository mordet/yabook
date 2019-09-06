use crate::handlers;
use crate::db;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    tables: Vec<db::table::Table>,
}

pub async fn handle_get() -> Result<Response, handlers::Error> {
    let mut pool = db::get_pool().get()?;
    let mut trx = pool.transaction()?;
    Ok(Response {
        tables: db::table::tables(&mut trx)
    })
}
