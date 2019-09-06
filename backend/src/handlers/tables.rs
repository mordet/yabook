use crate::handlers;
use crate::db;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    tables: Vec<db::table::Table>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    name: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    id: u64
}

pub async fn handle_get() -> Result<Response, handlers::Error> {
    let mut pool = db::get_pool().get()?;
    let mut trx = pool.transaction()?;
    Ok(Response {
        tables: db::table::tables(&mut trx)
    })
}

pub async fn handle_create(req: CreateRequest) -> Result<CreateResponse, handlers::Error> {
    let mut pool = db::get_pool().get()?;
    let mut trx = pool.transaction()?;
    let id = db::table::insert_table(&mut trx, &req.name, &req.location)?;
    trx.commit();
    Ok(CreateResponse {
        id: id
    })
}
