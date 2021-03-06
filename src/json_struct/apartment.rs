use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Apartment {
    pub id: i32,
    pub count_room: i32,
    pub price: f32,
}
