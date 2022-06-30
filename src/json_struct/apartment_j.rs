use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Apartment_j {
    pub hotj: serde_json::Value,
}
