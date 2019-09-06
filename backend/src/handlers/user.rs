use crate::handlers;
use crate::db;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    name: db::user::User
}

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
}

pub async fn handle_get(name: &str) -> Result<Response, handlers::Error> {
    let mut pool = db::get_pool().get()?;
    let mut trx = pool.transaction()?;
    let user = db::user::find_user(&mut trx, name).ok_or("Not found")?;
    Ok(Response{ name: user })
}

pub async fn handle_create(req: CreateRequest) -> Result<CreateResponse, handlers::Error> {
    Ok(CreateResponse{})
}
