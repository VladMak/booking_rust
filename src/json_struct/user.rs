use chrono::{Date, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct User_j {
    pub usej: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub password: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCheck {
    pub token: String,
}
