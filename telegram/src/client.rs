extern crate reqwest;

use serde_derive::{Serialize, Deserialize};

static URI: &str = "http://yabook.chebykin.org";

#[derive(Serialize,Deserialize)]
pub struct BookingListResponse {
}

pub fn get_bookings_list() -> Result<BookingListResponse, Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://yabook.chebykin.org/booking/list")?.text()?;

    println!("/booking/list response {}", &resp);

    Ok(serde_json::from_str(&resp)?)
}

#[derive(Serialize, Deserialize)]
pub struct CreateBookingRequest {
    table: String,
    owner: String,
    invite: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBookingResponse {
    id: i32,
    status: String
}

pub fn create_booking(table: String, owner: String, invite: Vec<String>)
    -> Result<CreateBookingResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request = CreateBookingRequest{table, owner, invite};

    let uri = URI.to_string() + "/booking/create";
    let resp = client
        .post(&uri)
        .body(serde_json::to_string(&request)?)
        .send()?.text()?;

    println!("/booking/create response {}", &resp);

    Ok(serde_json::from_str(&resp)?)
}
