use crate::handlers;
use crate::db;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
}

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    table: String,
    owner: String,
    invite: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    id: i32,
    status: String
}

#[derive(Serialize, Deserialize)]
pub struct DeleteResponse {
}

#[derive(Serialize, Deserialize)]
pub struct JoinRequest {
    booking_id: i32,
    login: String,
}

#[derive(Serialize, Deserialize)]
pub struct JoinResponse {
    status: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeclineRequest {
    booking_id: i32,
    login: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeclineResponse {
    status: String,
}

#[derive(Serialize, Deserialize)]
pub struct ItemResponse {
}

pub async fn handle_get(table: &str, from: i64, to: i64) -> Result<Response, handlers::Error> {
    Ok(Response{})
}

pub async fn handle_create(req: CreateRequest) -> Result<CreateResponse, handlers::Error> {
    Ok(CreateResponse{ id: 0, status: String::new() })
}

pub async fn handle_delete(id: i32) -> Result<DeleteResponse, handlers::Error> {
    Ok(DeleteResponse{})
}

pub async fn handle_join(req: JoinRequest) -> Result<JoinResponse, handlers::Error> {
    Ok(JoinResponse{ status: String::new() })
}

pub async fn handle_decline(req: DeclineRequest) -> Result<DeclineResponse, handlers::Error> {
    Ok(DeclineResponse{ status: String::new() })
}

pub async fn handle_item(id: i32) -> Result<ItemResponse, handlers::Error> {
    Ok(ItemResponse{})
}
