use crate::handlers;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Response {
    tables: models::Table

}

pub async fn handle_get() -> Result<(), handlers::Error> {
}
