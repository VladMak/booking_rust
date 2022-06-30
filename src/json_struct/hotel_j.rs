use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Hotel_j {
    pub hotj: serde_json::Value,
}
