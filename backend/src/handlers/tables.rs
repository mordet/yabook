use crate::handlers;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Table {
    table_id: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    tables: Vec<Table>,
}

pub async fn handle_get() -> Result<Response, handlers::Error> {
    Ok(Response {
        tables: vec![Table {
            table_id: "test".to_owned(),
            location: "spb".to_owned(),
        }],
    })
}
