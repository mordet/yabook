extern crate reqwest;

use serde_derive::{Serialize, Deserialize};
use chrono::Utc;

static URI: &str = "https://yabook.chebykin.org";

#[derive(Serialize,Deserialize)]
pub struct BookingListResponse {
}

pub fn get_bookings_list(table: String) -> Result<BookingListResponse, Box<dyn std::error::Error>> {
    let now = Utc::now().timestamp();
    let plus_hour = now + 3600;

    let uri = URI.to_string() + format!("/booking/list?table={}&from={}&to={}", table, now, plus_hour).as_str();

    let resp = reqwest::get(uri.as_str())?
        .error_for_status()?
        .text()?;

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
        .send()?
        .error_for_status()?
        .text()?;

    println!("/booking/create response: \"{}\"", &resp);

    Ok(serde_json::from_str(&resp)?)
}
