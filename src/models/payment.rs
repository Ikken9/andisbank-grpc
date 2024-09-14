use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: u32,
    pub loan_id: u32,
    pub user_id: u32,
    pub amount: f32,
    pub currency: String,
    pub payment_date: String,
    pub status: String
}