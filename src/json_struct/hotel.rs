use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Hotel {
    pub id: i32,
    pub name: String,
    pub stars: i32,
}
