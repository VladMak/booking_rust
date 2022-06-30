use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Organization {
    pub id: i32,
    pub name: String,
}
