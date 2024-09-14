use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub address: String
}