use chrono::{Date, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Booking {
    pub id: i32,
    pub hotelId: i32,
    pub apartmentId: i32,
    pub dateFrom: NaiveDate,
    pub dateTo: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Booking_j {
    pub booj: serde_json::Value,
}
