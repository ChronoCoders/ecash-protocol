use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TokenRecord {
    pub id: Uuid,
    pub serial_number: Vec<u8>,
    pub serial_hex: String,
    pub denomination: i64,
    pub currency: String,
    pub status: String,
    pub redeemed_at: DateTime<Utc>,
    pub merchant_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TransactionRecord {
    pub id: Uuid,
    pub transaction_type: String,
    pub amount: i64,
    pub denomination: i64,
    pub token_count: i32,
    pub institution_id: String,
    pub key_id: String,
    pub status: String,
    pub error_message: Option<String>,
    pub request_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}
